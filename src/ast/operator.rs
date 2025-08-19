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

use crate::{FuncParam, Statement, TypeName};

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    InfixOperator {
        symbol: String,
        another: FuncParam,
        return_type: TypeName,
        associativity: OperatorAssociativity,
        precedence: u8,
        body: Vec<Statement>,
    },
    PrefixOperator {
        symbol: String,
        another: FuncParam,
        return_type: TypeName,
        body: Vec<Statement>,
    },
    PostfixOperator {
        symbol: String,
        another: FuncParam,
        return_type: TypeName,
        body: Vec<Statement>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorAssociativity {
    Left,
    Right,
    None,
}
