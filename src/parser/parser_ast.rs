//
// Copyright 2025-2026 Shuntaro Kasatani
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

use crate::{InfixOperatorProperties, LiteralBind, Range};

#[derive(Debug, PartialEq, Clone)]
pub struct ParserProgram {
    pub statements: Vec<ParserStatement>,
}

#[derive(Debug, Clone)]
pub struct ParserStatement {
    pub range: Range,
    pub kind: ParserStatementKind,
}

impl PartialEq for ParserStatement {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range && self.kind == other.kind
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParserStatementKind {
    FuncDecl {
        required_by: Option<ParserSymbolPath>,
        name: String,
        params: Vec<ParserFuncParam>,
        return_type: Option<ParserSymbolPath>,
        body: Option<Vec<ParserStatement>>,
    },
    Return {
        value: Option<Vec<ExprToken>>,
    },
    Input {
        name: String,
        value_type: Option<ParserSymbolPath>,
        def_val: Option<Vec<ExprToken>>,
        attrs: Vec<ParserInputAttribute>,
    },
    Output {
        name: String,
        value_type: ParserSymbolPath,
    },
    Var {
        required_by: Option<ParserSymbolPath>,
        name: String,
        value_type: Option<ParserSymbolPath>,
        def_val: Option<Vec<ExprToken>>,
    },
    State {
        vars: Vec<ParserStateVar>,
    },
    Assign {
        target: ParserSymbolPath,
        value: Vec<ExprToken>,
    },
    FuncCall {
        name: ParserSymbolPath,
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
        inherits: Vec<ParserSymbolPath>,
        body: Vec<ParserStatement>,
    },
    ProtocolDecl {
        name: String,
        inherits: Vec<ParserSymbolPath>,
        body: Vec<ParserStatement>,
    },
    Init {
        required_by: Option<ParserSymbolPath>,
        literal_bind: Option<LiteralBind>,
        params: Vec<ParserFuncParam>,
        body: Option<Vec<ParserStatement>>,
    },
    InfixDefine {
        symbol: String,
        infix_properties: InfixOperatorProperties,
    },
    PrefixDefine {
        symbol: String,
    },
    OperatorFunc {
        op_type: ParserOperatorType,
        symbol: String,
        params: Vec<ParserFuncParam>,
        return_type: ParserSymbolPath,
        body: Vec<ParserStatement>,
    },
    Block {
        statements: Vec<ParserStatement>,
    },
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum ParserOperatorType {
    Infix,
    Prefix,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserInputAttribute {
    pub name: String,
    pub args: Vec<Vec<ExprToken>>,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserFuncCallArg {
    pub label: Option<String>,
    pub value: Vec<ExprToken>,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserStateVar {
    pub name: String,
    pub value_type: Option<ParserSymbolPath>,
    pub def_val: Vec<ExprToken>,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserFuncParam {
    pub label: Option<String>,
    pub name: String,
    pub value_type: Option<ParserSymbolPath>,
    pub def_val: Option<Vec<ExprToken>>,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprToken {
    pub range: Range,
    pub kind: ExprTokenKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprTokenKind {
    IntLiteral(u32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    Operator(String),
    Identifier(ParserSymbolPath),
    FuncCall {
        path: ParserSymbolPath,
        args: Vec<ParserFuncCallArg>,
    },
    LParen,
    RParen,
}

pub type ParserSymbolPath = Vec<ParserSymbolPathComponent>;

#[derive(Debug, PartialEq, Clone)]
pub struct ParserSymbolPathComponent {
    pub range: Range,
    pub symbol: String,
}
