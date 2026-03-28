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

use crate::ast::{FuncParam, NameSpaceID, Range, symbol_table::Block, type_registry::ResolvedType};
use hashbrown::Equivalent;

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct PrefixOperator {
    pub symbol: String,
    pub namespace_id: NameSpaceID,
    pub operand: FuncParam,
    pub return_type: ResolvedType,
    pub block: Block,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone, Eq, serde::Serialize)]
pub struct PrefixOperatorProperties {
    pub precedence: u32,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, serde::Serialize)]
pub struct PrefixQuery {
    pub symbol: String,
    pub operand_type: ResolvedType,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct PrefixQueryRef<'a> {
    pub symbol: &'a str,
    pub operand_type: &'a ResolvedType,
}

impl Equivalent<PrefixQuery> for PrefixQueryRef<'_> {
    fn equivalent(&self, key: &PrefixQuery) -> bool {
        self.symbol == key.symbol && self.operand_type == key.operand_type
    }
}
