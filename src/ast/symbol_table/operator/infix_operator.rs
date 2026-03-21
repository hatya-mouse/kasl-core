use crate::{FuncParam, NameSpaceID, Range, symbol_table::Block, type_registry::ResolvedType};
use hashbrown::Equivalent;

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct InfixOperator {
    pub symbol: String,
    pub namespace_id: NameSpaceID,
    pub lhs: FuncParam,
    pub rhs: FuncParam,
    pub return_type: ResolvedType,
    pub block: Block,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone, Eq, serde::Serialize)]
pub struct InfixOperatorProperties {
    pub precedence: u32,
    pub associativity: OperatorAssociativity,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone, Eq, serde::Serialize)]
pub enum OperatorAssociativity {
    Left,
    Right,
    None,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, serde::Serialize)]
pub struct InfixQuery {
    pub symbol: String,
    pub lhs_type: ResolvedType,
    pub rhs_type: ResolvedType,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct InfixQueryRef<'a> {
    pub symbol: &'a str,
    pub lhs_type: &'a ResolvedType,
    pub rhs_type: &'a ResolvedType,
}

impl Equivalent<InfixQuery> for InfixQueryRef<'_> {
    fn equivalent(&self, key: &InfixQuery) -> bool {
        self.symbol == key.symbol && self.lhs_type == key.lhs_type && self.rhs_type == key.rhs_type
    }
}
