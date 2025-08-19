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

use crate::{Expression, FuncCallArg, FuncParam, InputAttribute, StateVar, SymbolPath, TypeName};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    FuncDecl {
        required_by: Option<String>,
        name: String,
        params: Vec<FuncParam>,
        return_type: TypeName,
        body: Vec<Statement>,
    },
    Return {
        value: Option<Expression>,
    },
    Input {
        name: String,
        value_type: TypeName,
        def_val: Option<Expression>,
        attrs: Vec<InputAttribute>,
    },
    Output {
        name: String,
        value_type: TypeName,
    },
    Var {
        name: String,
        value_type: TypeName,
        def_val: Expression,
    },
    State {
        vars: Vec<StateVar>,
    },
    Assign {
        target: SymbolPath,
        value: Expression,
    },
    FuncCall {
        name: Vec<String>,
        args: Vec<FuncCallArg>,
    },
    If {
        condition: Expression,
        body: Vec<Statement>,
    },
    IfElse {
        condition: Expression,
        body: Vec<Statement>,
        else_body: Vec<Statement>,
    },
    StructDecl {
        name: String,
        inherits: Vec<TypeName>,
        body: Vec<Statement>,
    },
    ProtocolDecl {
        name: String,
        inherits: Vec<TypeName>,
        body: Vec<Statement>,
    },
}
