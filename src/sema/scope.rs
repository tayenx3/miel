use std::collections::{HashMap, HashSet};
use crate::common::Span;
use super::types::TypeId;
use super::constants::ConstValue;

#[derive(Debug)]
pub struct ScopeManager {
    root_scope: SymbolMap,
    scopes: Vec<FunctionScope>,
}

impl ScopeManager {
    pub fn new() -> Self {
        Self {
            root_scope: SymbolMap::new(SymbolMapKind::Default),
            scopes: Vec::new(),
        }
    }
    
    pub fn enter_scope(&mut self, ret_ty: TypeId, sig_span: Span) {
        self.scopes.push(FunctionScope::new(ret_ty, sig_span));
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn find_symbol(&self, name: &lasso::Spur) -> Option<Symbol<'_>> {
        if let Some(s) = self.scopes.last().and_then(|s| s.find_symbol(name)) { Some(s) }
        else if let Some(s) = self.root_scope.find_symbol(name) { Some(s) }
        else { None }
    }
    
    pub fn find_symbol_mut(&mut self, name: &lasso::Spur) -> Option<SymbolMut<'_>> {
        if let Some(s) = self.scopes.last_mut().and_then(|s| s.find_symbol_mut(name)) { Some(s) }
        else if let Some(s) = self.root_scope.find_symbol_mut(name) { Some(s) }
        else { None }
    }
}

#[derive(Debug)]
pub struct FunctionScope {
    smaps: Vec<SymbolMap>,
    pub ret_ty: TypeId,
    pub sig_span: Span,
}

impl FunctionScope {
    pub fn new(ret_ty: TypeId, sig_span: Span) -> Self {
        Self {
            smaps: Vec::new(),
            ret_ty, sig_span,
        }
    }

    pub fn enter_smap(&mut self, kind: SymbolMapKind) {
        self.smaps.push(SymbolMap::new(kind));
    }

    pub fn exit_smap(&mut self) {
        self.smaps.pop();
    }
    
    pub fn find_symbol(&self, name: &lasso::Spur) -> Option<Symbol<'_>> {
        for map in self.smaps.iter().rev() {
            if let Some(s) = map.find_symbol(name) {
                return Some(s);
            }
        }
        None
    }
    
    pub fn find_symbol_mut(&mut self, name: &lasso::Spur) -> Option<SymbolMut<'_>> {
        for map in self.smaps.iter_mut().rev() {
            if let Some(s) = map.find_symbol_mut(name) {
                return Some(s);
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolMapKind {
    Default, Loop
}

#[derive(Debug)]
pub struct SymbolMap {
    pub kind: SymbolMapKind,
    defined_symbols: HashSet<lasso::Spur>,
    types: HashMap<lasso::Spur, TypeId>,
    def_locs: HashMap<lasso::Spur, Span>,
    constants: HashMap<lasso::Spur, ConstValue>,
}

impl SymbolMap {
    pub fn new(kind: SymbolMapKind) -> Self {
        Self {
            kind,
            defined_symbols: HashSet::new(),
            types: HashMap::new(),
            def_locs: HashMap::new(),
            constants: HashMap::new(),
        }
    }

    pub fn is_loop(&self) -> bool {
        matches!(&self.kind, SymbolMapKind::Loop)
    }
    
    pub fn define_symbol(&mut self, name: lasso::Spur, ty: TypeId, defined_at: Span) {
        self.defined_symbols.insert(name);
        self.types.insert(name, ty);
        self.def_locs.insert(name, defined_at);
    }

    pub fn define_const_symbol(&mut self, name: lasso::Spur, ty: TypeId, defined_at: Span, c: ConstValue) {
        self.defined_symbols.insert(name);
        self.types.insert(name, ty);
        self.def_locs.insert(name, defined_at);
        self.constants.insert(name, c);
    }

    pub fn find_symbol(&self, name: &lasso::Spur) -> Option<Symbol<'_>> {
        if self.defined_symbols.contains(name) {
            Some(Symbol {
                ty: &self.types[name],
                val: self.constants.get(name)
            })
        } else {
            None
        }
    }

    pub fn find_symbol_mut(&mut self, name: &lasso::Spur) -> Option<SymbolMut<'_>> {
        if self.defined_symbols.contains(name) {
            Some(SymbolMut {
                ty: self.types.get_mut(name).unwrap(),
                val: self.constants.get_mut(name)
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Copy)]
pub struct Symbol<'a> {
    pub ty: &'a TypeId,
    pub val: Option<&'a ConstValue>,
}

impl Clone for Symbol<'_> {
    fn clone(&self) -> Self {
        Self {
            ty: self.ty,
            val: self.val,
        }
    }
}

#[derive(Debug)]
pub struct SymbolMut<'a> {
    pub ty: &'a mut TypeId,
    pub val: Option<&'a mut ConstValue>,
}