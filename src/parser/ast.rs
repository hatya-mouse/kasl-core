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

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    FuncDecl {
        name: String,
        params: Vec<FuncParam>,
        return_type: Option<String>,
        body: Vec<Statement>,
    },
    Return {
        value: Vec<ExprToken>,
    },
    Input {
        name: String,
        value_type: Option<String>,
        def_val: Option<Vec<ExprToken>>,
        attrs: Vec<InputAttribute>,
    },
    Output {
        name: String,
        value_type: String,
    },
    Var {
        name: String,
        value_type: Option<String>,
        def_val: Vec<ExprToken>,
    },
    State {
        vars: Vec<StateVar>,
    },
    Assign {
        property: Vec<String>,
        value: Vec<ExprToken>,
    },
    FuncCall {
        name: Vec<String>,
        args: Vec<Vec<ExprToken>>,
    },
    If {
        condition: Vec<ExprToken>,
        body: Vec<Statement>,
    },
    IfElse {
        condition: Vec<ExprToken>,
        body: Vec<Statement>,
        else_body: Vec<Statement>,
    },
    StructDecl {
        name: String,
        inherits: Vec<String>,
        body: Vec<Statement>,
    },
    ProtocolDecl {
        name: String,
        inherits: Vec<String>,
        requires: Vec<ProtocolRequirement>,
    },
    Init {
        literal_bind: Option<LiteralBind>,
        params: Vec<FuncParam>,
        body: Vec<Statement>,
    },
    Infix {
        symbol: String,
        params: Vec<FuncParam>,
        return_type: String,
        attrs: HashMap<String, String>,
        body: Vec<Statement>,
    },
    Prefix {
        symbol: String,
        params: Vec<FuncParam>,
        return_type: String,
        body: Vec<Statement>,
    },
    Postfix {
        symbol: String,
        params: Vec<FuncParam>,
        return_type: String,
        body: Vec<Statement>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct FuncParam {
    pub label: Option<String>,
    pub name: String,
    pub value_type: Option<String>,
    pub def_val: Option<Vec<ExprToken>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InputAttribute {
    pub name: String,
    pub args: Vec<Vec<ExprToken>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ProtocolRequirement {
    Func {
        name: String,
        params: Vec<FuncParam>,
        return_type: Option<String>,
    },
    Infix {
        symbol: String,
        params: Vec<FuncParam>,
        return_type: String,
        attrs: HashMap<String, String>,
    },
    Prefix {
        symbol: String,
        params: Vec<FuncParam>,
        return_type: String,
    },
    Postfix {
        symbol: String,
        params: Vec<FuncParam>,
        return_type: String,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct StateVar {
    pub name: String,
    pub value_type: Option<String>,
    pub def_val: Vec<ExprToken>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralBind {
    IntLiteral,
    FloatLiteral,
    BoolLiteral,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprToken {
    Literal(String),
    Operator(String),
    Identifier(Vec<String>),
    FuncCall {
        name: Vec<String>,
        args: Vec<Vec<ExprToken>>,
    },
}
