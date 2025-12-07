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

use crate::{FuncParam, Statement, TypeDef};

#[derive(Debug, PartialEq, Clone)]
pub enum Operator<'a> {
    InfixOperator {
        symbol: String,
        another: FuncParam<'a>,
        return_type: Option<&'a TypeDef<'a>>,
        associativity: OperatorAssociativity,
        precedence: u8,
        body: Vec<Statement<'a>>,
    },
    PrefixOperator {
        symbol: String,
        another: FuncParam<'a>,
        return_type: Option<&'a TypeDef<'a>>,
        body: Vec<Statement<'a>>,
    },
    PostfixOperator {
        symbol: String,
        another: FuncParam<'a>,
        return_type: Option<&'a TypeDef<'a>>,
        body: Vec<Statement<'a>>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorAssociativity {
    Left,
    Right,
    None,
}
