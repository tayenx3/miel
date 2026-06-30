use std::collections::HashMap;
use crate::{common::{Diag, Label}, parser::ast::{NodeId, ParsedType, ParsedTypeKind}};

pub type TypePool = HashMap<TypeId, Type>;
pub type TypeRegistry = HashMap<lasso::Spur, TypeId>;
pub type TypeMap = HashMap<NodeId, TypeId>;

/// sets up all of the primitive types and returns the next valid type ID along with a [`PreDefinedTypes`]
fn setup_primitives(
    pool: &mut TypePool,
    registry: &mut TypeRegistry,
    rodeo: &mut lasso::Rodeo
) -> (PreDefinedTypes, usize) {
    let mut next_type_id = 0;

    let mut repeated_types = [
        ("int", Type::Int),
        ("i8", Type::I8),
        ("i16", Type::I16),
        ("i32", Type::I32),
        ("i64", Type::I64),
        ("uint", Type::UInt),
        ("u8", Type::U8),
        ("u16", Type::U16),
        ("u32", Type::U32),
        ("u64", Type::U64),
        ("float", Type::Float),
        ("f32", Type::F32),
        ("f64", Type::F64),
    ];

    for (name, ty) in repeated_types {
        let id = TypeId(next_type_id);
        next_type_id += 1;
        pool.insert(id, ty);
        registry.insert(rodeo.get_or_intern_static(name), id);
    }
    
    let bool_id = TypeId(next_type_id);
    next_type_id += 1;
    pool.insert(bool_id, Type::Bool);
    registry.insert(rodeo.get_or_intern_static("bool"), bool_id);
    
    let nil_id = TypeId(next_type_id);
    next_type_id += 1;
    pool.insert(nil_id, Type::Nil);
    registry.insert(rodeo.get_or_intern_static("nil"), nil_id);
    
    let never_id = TypeId(next_type_id);
    next_type_id += 1;
    pool.insert(never_id, Type::Never);

    (PreDefinedTypes {
        bool_id,
        nil_id,
        never_id,
    }, next_type_id)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PreDefinedTypes {
    pub bool_id: TypeId,
    pub nil_id: TypeId,
    pub never_id: TypeId,
}

#[derive(Debug, Clone)]
pub struct TypeManager {
    pool: TypePool,
    next_type_id: usize,
    registry: TypeRegistry,
    coercions: HashMap<TypeId, TypeId>,
    type_map: TypeMap,
    pub predef_types: PreDefinedTypes,
}

impl TypeManager {
    pub fn new(rodeo: &mut lasso::Rodeo) -> Self {
        let mut pool = TypePool::new();
        let mut registry = TypeRegistry::new();
        let (predef_types, next_type_id) = setup_primitives(&mut pool, &mut registry, rodeo);
        Self {
            pool,
            next_type_id,
            registry,
            coercions: HashMap::new(),
            type_map: TypeMap::new(),
            predef_types,
        }
    }

    pub fn resolve_type(&mut self, ty: &ParsedType) -> Result<TypeId, Vec<Diag>> {
        match &ty.kind {
            ParsedTypeKind::Identifier(s) =>
                self.registry.get(s)
                    .copied()
                    .ok_or(vec![
                        Diag::error()
                            .with_message("Unknown type")
                            .with_labels(vec![
                                Label::primary(ty.span.source_id, ty.span.start..ty.span.end)
                                    .with_message("type not defined")
                            ])
                    ]),
            ParsedTypeKind::Nil => Ok(self.predef_types.nil_id),
            ParsedTypeKind::Tuple(items) => {
                let mut errors = Vec::new();
                let mut item_tys = Vec::new();
                for item in items {
                    match self.resolve_type(item) {
                        Ok(ty) => item_tys.push(ty),
                        Err(errs) => errors.extend(errs),
                    }
                }
                if errors.is_empty() {
                    Ok(self.create_type(Type::Tuple(item_tys)))
                } else {
                    Err(errors)
                }
            },
        }
    }

    pub fn create_type(&mut self, ty: Type) -> TypeId {
        let id = TypeId(self.next_type_id);
        self.next_type_id += 1;
        self.pool.insert(id, ty);
        id
    }

    pub fn name_type(&mut self, name: lasso::Spur, id: TypeId) {
        self.registry.insert(name, id);
    }
    
    pub fn can_coerce(&self, from: &TypeId, into: &TypeId) -> bool {
        let from_ty = self.get_type(from).unwrap();
        let into_ty = self.get_type(into).unwrap();
        from_ty.is_coerceable_into(into_ty)
    }

    pub fn get_type(&self, id: &TypeId) -> Option<&Type> {
        let mut current_id = id;
        while let Some(id) = self.coercions.get(current_id) {
            current_id = id;
        }
        self.pool.get(current_id)
    }

    pub fn coerce_type(&mut self, from: TypeId, into: TypeId) {
        self.coercions.insert(from, into);
    }

    pub fn assign_node_type(&mut self, node_id: NodeId, ty: TypeId) {
        debug_assert!(!self.type_map.contains_key(&node_id));
        self.type_map.insert(node_id, ty);
    }
    
    pub fn get_node_type(&mut self, node_id: &NodeId) -> Option<&TypeId> {
        self.type_map.get(node_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    AmbiguousInt,
    Int, UInt, // platform integer
    I8, I16, I32, I64,
    U8, U16, U32, U64,
    AmbiguousFloat,
    Float, // platform float
    F32, F64,
    Bool,
    Nil,
    Never,
    Tuple(Vec<TypeId>),
    Callable {
        params: Vec<TypeId>,
        ret_ty: TypeId
    },
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::AmbiguousInt | Self::Int
            | Self::I8 | Self::I16 | Self::I32 | Self::I64
            | Self::U8 | Self::U16 | Self::U32 | Self::U64
            | Self::AmbiguousFloat | Self::Float | Self::F32 | Self::F64
            | Self::Never
        )
    }

    pub fn is_int(&self) -> bool {
        matches!(
            self,
            Self::AmbiguousInt | Self::Int
            | Self::I8 | Self::I16 | Self::I32 | Self::I64
            | Self::U8 | Self::U16 | Self::U32 | Self::U64
            | Self::Never
        )
    }
    
    pub fn is_float(&self) -> bool {
        matches!(
            self,
            Self::AmbiguousFloat | Self::Float
            | Self::F32 | Self::F64
            | Self::Never
        )
    }

    /// this checks if `self` can be coerced *into* `other`
    pub fn is_coerceable_into(&self, other: &Type) -> bool {
        *other == Type::Never
        || self == other
        || (*self == Type::AmbiguousInt && other.is_int())
        || (*self == Type::AmbiguousFloat && other.is_float())
    }
}

impl Type {
    pub fn format(&self, types: &TypeManager) -> String {
        match self {
            Self::AmbiguousInt => "<int>".to_string(),
            Self::Int => "int".to_string(),
            Self::UInt => "uint".to_string(),
            Self::I8 => "i8".to_string(),
            Self::I16 => "i16".to_string(),
            Self::I32 => "i32".to_string(),
            Self::I64 => "i64".to_string(),
            Self::U8 => "u8".to_string(),
            Self::U16 => "u16".to_string(),
            Self::U32 => "u32".to_string(),
            Self::U64 => "u64".to_string(),
            Self::AmbiguousFloat => "<float>".to_string(),
            Self::Float => "float".to_string(),
            Self::F32 => "f32".to_string(),
            Self::F64 => "f64".to_string(),
            Self::Bool => "bool".to_string(),
            Self::Nil => "nil".to_string(),
            Self::Never => "!".to_string(),
            Self::Tuple(items) => {
                let mut buf = String::new();
                for (idx, item) in items.iter().enumerate() {
                    if idx > 0 {
                        buf.push_str(", ");
                    }
                    buf.push_str(&types.get_type(item).unwrap().format(types));
                }
                if items.len() < 2 {
                    buf.push_str(", ");
                }
                format!("({buf})")
            },
            Self::Callable { params, ret_ty } => {
                let mut buf = String::new();
                for (idx, item) in params.iter().enumerate() {
                    if idx > 0 {
                        buf.push_str(", ");
                    }
                    buf.push_str(
                        &types.get_type(item)
                            .unwrap().format(types)
                    );
                }
                format!(
                    "callable({buf}): {}",
                    types.get_type(ret_ty)
                        .unwrap().format(types)
                )
            },
        }
    }
}