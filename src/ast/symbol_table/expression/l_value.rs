use crate::{Expr, VariableID, type_registry::ResolvedType};

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct LValue {
    pub kind: LValueKind,
    pub value_type: ResolvedType,
}

impl LValue {
    pub fn new(kind: LValueKind, value_type: ResolvedType) -> Self {
        Self { kind, value_type }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum LValueKind {
    Identifier(VariableID),
    StructField { lhs: Box<LValue>, offset: i32 },
    Subscript { lhs: Box<LValue>, index: Box<Expr> },
}
