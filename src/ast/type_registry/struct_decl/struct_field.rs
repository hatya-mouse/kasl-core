use crate::{Expr, Range, type_registry::ResolvedType};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct StructField {
    pub name: String,
    pub value_type: ResolvedType,
    pub def_val: Expr,
    pub range: Range,
}
