use crate::{Expr, Range, type_registry::ResolvedType};

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct ScopeVar {
    pub name: String,
    pub value_type: ResolvedType,
    pub def_val: Option<Expr>,
    pub range: Range,
    pub var_kind: VariableKind,
}

impl ScopeVar {
    pub fn expect_def_val(&self) -> &Expr {
        self.def_val.as_ref().unwrap()
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum VariableKind {
    Input { attrs: Vec<InputAttribute> },
    Output,
    State,
    GlobalConst,
    FuncParam,
    SelfParam,
    LocalVar,
    LocalConst,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct InputAttribute {
    pub name: String,
    pub args: Vec<Expr>,
    pub range: Range,
}
