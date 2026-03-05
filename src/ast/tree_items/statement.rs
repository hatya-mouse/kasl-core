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

use crate::{Expression, FuncCallArg, data::SymbolID};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Return {
        value: Option<Expression>,
    },
    VarDecl {
        name: String,
        value_type: SymbolID,
        def_val: Expression,
    },
    Assign {
        target: SymbolID,
        value: Expression,
    },
    FuncCall {
        path: SymbolID,
        args: Vec<FuncCallArg>,
    },
    If {
        main: IfArm,
        else_ifs: Vec<IfArm>,
        else_body: Vec<Statement>,
    },
    Block {
        body: Vec<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfArm {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

impl IfArm {
    pub fn new(condition: Expression, body: Vec<Statement>) -> Self {
        Self { condition, body }
    }
}
