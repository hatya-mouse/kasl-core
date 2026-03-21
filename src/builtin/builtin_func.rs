use crate::type_registry::ResolvedType;
use cranelift::prelude::FunctionBuilder;
use cranelift_codegen::ir;
use std::fmt::Display;

pub type BuiltinFuncTranslator = Box<dyn Fn(&mut FunctionBuilder, &[ir::Value]) -> ir::Value>;

pub struct BuiltinFunc {
    pub name: &'static str,
    pub params: Vec<ResolvedType>,
    pub return_type: ResolvedType,
    pub translator: BuiltinFuncTranslator,
}

/// An ID used to identify an builtin function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, serde::Serialize)]
pub struct BuiltinFuncID(usize);

impl BuiltinFuncID {
    pub fn new(val: usize) -> Self {
        Self(val)
    }
}

impl Display for BuiltinFuncID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
