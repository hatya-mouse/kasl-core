use crate::{Expression, Type, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Time,
    Input,
    Output,
    Variable,
    Function,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub kind: SymbolKind,
    pub data_type: Type,
    pub initial_value: Option<Expression>,
    pub range: Option<(f32, f32)>,
    pub value: Option<Value>,
}
