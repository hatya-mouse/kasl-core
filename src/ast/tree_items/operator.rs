//
// © 2025-2026 Shuntaro Kasatani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::fmt::Debug;

use crate::{FuncParam, Statement, SymbolPath};

#[derive(Debug, PartialEq, Clone)]
pub struct InfixOperator {
    pub symbol: String,
    pub lhs: FuncParam,
    pub rhs: FuncParam,
    pub return_type: SymbolPath,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixOperator {
    pub symbol: String,
    pub operand: FuncParam,
    pub return_type: SymbolPath,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct InfixOperatorProperties {
    pub precedence: u32,
    pub associativity: OperatorAssociativity,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum OperatorAssociativity {
    Left,
    Right,
    None,
}
