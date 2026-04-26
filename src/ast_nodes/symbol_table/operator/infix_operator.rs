//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::ast_nodes::{
    FuncParam, NameSpaceID, Range, symbol_table::Block, type_registry::ResolvedType,
};
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
