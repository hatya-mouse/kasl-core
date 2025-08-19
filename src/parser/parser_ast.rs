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
pub struct ParserProgram {
    pub statements: Vec<ParserStatement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParserStatement {
    FuncDecl {
        required_by: Option<String>,
        name: String,
        params: Vec<ParserFuncParam>,
        return_type: Option<String>,
        body: Vec<ParserStatement>,
    },
    Return {
        value: Option<Vec<ExprToken>>,
    },
    Input {
        name: String,
        value_type: Option<String>,
        def_val: Option<Vec<ExprToken>>,
        attrs: Vec<ParserInputAttribute>,
    },
    Output {
        name: String,
        value_type: String,
    },
    Var {
        required_by: Option<String>,
        name: String,
        value_type: Option<String>,
        def_val: Vec<ExprToken>,
    },
    State {
        vars: Vec<ParserStateVar>,
    },
    Assign {
        target: Vec<String>,
        value: Vec<ExprToken>,
    },
    FuncCall {
        name: Vec<String>,
        args: Vec<ParserFuncCallArg>,
    },
    If {
        condition: Vec<ExprToken>,
        body: Vec<ParserStatement>,
    },
    IfElse {
        condition: Vec<ExprToken>,
        body: Vec<ParserStatement>,
        else_body: Vec<ParserStatement>,
    },
    StructDecl {
        name: String,
        inherits: Vec<String>,
        body: Vec<ParserStatement>,
    },
    ProtocolDecl {
        name: String,
        inherits: Vec<String>,
        requires: Vec<ParserProtocolRequirement>,
    },
    Init {
        literal_bind: Option<ParserLiteralBind>,
        params: Vec<ParserFuncParam>,
        body: Vec<ParserStatement>,
    },
    Infix {
        symbol: String,
        params: Vec<ParserFuncParam>,
        return_type: String,
        attrs: HashMap<String, ParserInfixAttrValue>,
        body: Vec<ParserStatement>,
    },
    Prefix {
        symbol: String,
        params: Vec<ParserFuncParam>,
        return_type: String,
        body: Vec<ParserStatement>,
    },
    Postfix {
        symbol: String,
        params: Vec<ParserFuncParam>,
        return_type: String,
        body: Vec<ParserStatement>,
    },
    Block {
        statements: Vec<ParserStatement>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserInputAttribute {
    pub name: String,
    pub args: Vec<Vec<ExprToken>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserFuncCallArg {
    pub label: Option<String>,
    pub value: Vec<ExprToken>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParserProtocolRequirement {
    Func {
        required_by: Option<String>,
        name: String,
        params: Vec<ParserFuncParam>,
        return_type: Option<String>,
    },
    Var {
        required_by: Option<String>,
        name: String,
        value_type: Option<String>,
        def_val: Option<Vec<ExprToken>>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserStateVar {
    pub name: String,
    pub value_type: Option<String>,
    pub def_val: Vec<ExprToken>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserFuncParam {
    pub label: Option<String>,
    pub name: String,
    pub value_type: Option<String>,
    pub def_val: Option<Vec<ExprToken>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParserInfixAttrValue {
    String(String),
    Integer(u32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParserLiteralBind {
    IntLiteral,
    FloatLiteral,
    BoolLiteral,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprToken {
    IntLiteral(u32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    Operator(String),
    Identifier(Vec<String>),
    FuncCall {
        name: Vec<String>,
        args: Vec<ParserFuncCallArg>,
    },
}
