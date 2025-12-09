//
// Copyright 2025 Shuntaro Kasatani
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
pub struct Operator {
    pub symbol: String,
    pub return_type: Option<SymbolPath>,
    pub body: Vec<Statement>,
    pub kind: OperatorKind,
}

#[derive(PartialEq, Clone)]
pub enum OperatorKind {
    InfixOperator {
        another: FuncParam,
        associativity: OperatorAssociativity,
        precedence: u8,
    },
    PrefixOperator,
    PostfixOperator,
}

impl Debug for OperatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorKind::InfixOperator { .. } => write!(f, "infix"),
            OperatorKind::PrefixOperator => write!(f, "prefix"),
            OperatorKind::PostfixOperator => write!(f, "postfix"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorAssociativity {
    Left,
    Right,
    None,
}
