use std::collections::HashMap;

// ohno
pub struct TypePool {
    types: HashMap<TypeId, Type>,
    next_id: usize,
    
    pub int_id: TypeId,
    pub uint_id: TypeId,
    pub i8_id: TypeId,
    pub i16_id: TypeId,
    pub i32_id: TypeId,
    pub i64_id: TypeId,
    pub u8_id: TypeId,
    pub u16_id: TypeId,
    pub u32_id: TypeId,
    pub u64_id: TypeId,
    pub float_id: TypeId,
    pub f32_id: TypeId,
    pub f64_id: TypeId,
    pub nil_id: TypeId,
}

impl TypePool {
    pub fn new() -> Self {
        let mut types = HashMap::new();
        let mut next_id = 0usize;

        let int_id = TypeId(next_id);
        next_id += 1;
        types.insert(int_id, Type::Int);
        
        let uint_id = TypeId(next_id);
        next_id += 1;
        types.insert(uint_id, Type::UInt);
        
        let i8_id = TypeId(next_id);
        next_id += 1;
        types.insert(i8_id, Type::I8);
        
        let i16_id = TypeId(next_id);
        next_id += 1;
        types.insert(i16_id, Type::I16);
        
        let i32_id = TypeId(next_id);
        next_id += 1;
        types.insert(i32_id, Type::I32);
        
        let i64_id = TypeId(next_id);
        next_id += 1;
        types.insert(i64_id, Type::I64);
        
        let u8_id = TypeId(next_id);
        next_id += 1;
        types.insert(u8_id, Type::U8);
        
        let u16_id = TypeId(next_id);
        next_id += 1;
        types.insert(u16_id, Type::U16);
        
        let u32_id = TypeId(next_id);
        next_id += 1;
        types.insert(u32_id, Type::U32);
        
        let u64_id = TypeId(next_id);
        next_id += 1;
        types.insert(u64_id, Type::U64);
        
        let float_id = TypeId(next_id);
        next_id += 1;
        types.insert(float_id, Type::Float);
        
        let f32_id = TypeId(next_id);
        next_id += 1;
        types.insert(f32_id, Type::F32);
        
        let f64_id = TypeId(next_id);
        next_id += 1;
        types.insert(f64_id, Type::F64);
        
        let nil_id = TypeId(next_id);
        next_id += 1;
        types.insert(nil_id, Type::Nil);

        Self {
            types, next_id,
            
            int_id, uint_id,
            i8_id, i16_id, i32_id, i64_id,
            u8_id, u16_id, u32_id, u64_id,
            float_id, f32_id, f64_id,
            nil_id
        }
    }
    
    pub fn create_type(&mut self, ty: Type) -> TypeId {
        let id = TypeId(self.next_id);
        self.next_id += 1;
        self.types.insert(id, ty);
        id
    }

    pub fn coerce_type(&mut self, from: &TypeId, to: Type) {
        *self.types.get_mut(from).unwrap() = to;
    }

    pub fn get_type(&self, id: &TypeId) -> Option<&Type> {
        self.types.get(id)
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
    Nil,
    Tuple(Vec<TypeId>),
    Callable {
        params: Vec<TypeId>,
        ret_ty: TypeId
    }
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::AmbiguousInt | Self::Int
            | Self::I8 | Self::I16 | Self::I32 | Self::I64
            | Self::U8 | Self::U16 | Self::U32 | Self::U64
            | Self::AmbiguousFloat | Self::Float | Self::F32 | Self::F64
        )
    }

    pub fn is_int(&self) -> bool {
        matches!(
            self,
            Self::AmbiguousInt | Self::Int
            | Self::I8 | Self::I16 | Self::I32 | Self::I64
            | Self::U8 | Self::U16 | Self::U32 | Self::U64
        )
    }
    
    pub fn is_float(&self) -> bool {
        matches!(
            self,
            Self::AmbiguousFloat | Self::Float | Self::F32 | Self::F64
        )
    }

    /// this checks if `self` can be coerced *into* `other`
    pub fn is_coerceable_into(&self, other: &Type) -> bool {
        self == other
        || (*self == Type::AmbiguousInt && other.is_int())
        || (*self == Type::AmbiguousFloat && other.is_float())
    }
}

impl Type {
    pub fn format(&self, type_pool: &TypePool) -> String {
        match self {
            Self::AmbiguousInt => "<int>".to_string(),
            Self::Int => "int".to_string(),
            Self::UInt => "uint".to_string(),
            Self::I8 => "i8".to_string(),
            Self::I16 => "i16".to_string(),
            Self::I32 => "i32".to_string(),
            Self::I64 => "i16".to_string(),
            Self::U8 => "u8".to_string(),
            Self::U16 => "u16".to_string(),
            Self::U32 => "u32".to_string(),
            Self::U64 => "u16".to_string(),
            Self::AmbiguousFloat => "float".to_string(),
            Self::Float => "float".to_string(),
            Self::F32 => "f32".to_string(),
            Self::F64 => "f64".to_string(),
            Self::Nil => "nil".to_string(),
            Self::Tuple(items) => {
                let mut buf = String::new();
                for (idx, item) in items.iter().enumerate() {
                    if idx > 0 {
                        buf.push_str(", ");
                    }
                    buf.push_str(&type_pool.get_type(item).unwrap().format(type_pool));
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
                        &type_pool.get_type(item)
                            .unwrap().format(type_pool)
                    );
                }
                format!(
                    "callable({buf}): {}",
                    type_pool.get_type(ret_ty)
                        .unwrap().format(type_pool)
                )
            },
        }
    }
}