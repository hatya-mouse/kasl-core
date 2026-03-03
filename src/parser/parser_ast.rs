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

use std::fmt::Display;

use crate::{InfixOperatorProperties, LiteralBind, Range};

#[derive(Debug, PartialEq, Clone)]
pub struct ParserProgram {
    pub statements: Vec<ParserTopLevelStmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserTopLevelStmt {
    pub range: Range,
    pub kind: ParserTopLevelStmtKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserBodyStmt {
    pub range: Range,
    pub kind: ParserBodyStmtKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParserTopLevelStmtKind {
    FuncDecl {
        required_by: Option<ParserSymbolPath>,
        name: String,
        params: Vec<ParserFuncParam>,
        return_type: Option<ParserSymbolPath>,
        body: Option<Vec<ParserBodyStmt>>,
    },
    Input {
        name: String,
        value_type: Option<ParserSymbolPath>,
        def_val: Vec<ExprToken>,
        attrs: Vec<ParserInputAttribute>,
    },
    Output {
        name: String,
        value_type: Option<ParserSymbolPath>,
        def_val: Vec<ExprToken>,
    },
    GlobalVar {
        required_by: Option<ParserSymbolPath>,
        name: String,
        value_type: Option<ParserSymbolPath>,
        def_val: Vec<ExprToken>,
    },
    State {
        vars: Vec<ParserStateVar>,
    },
    StructDecl {
        name: String,
        inherits: Vec<ParserSymbolPath>,
        body: Vec<ParserTopLevelStmt>,
    },
    ProtocolDecl {
        name: String,
        inherits: Vec<ParserSymbolPath>,
        body: Vec<ParserTopLevelStmt>,
    },
    Init {
        required_by: Option<ParserSymbolPath>,
        literal_bind: Option<LiteralBind>,
        params: Vec<ParserFuncParam>,
        body: Option<Vec<ParserBodyStmt>>,
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
        body: Vec<ParserBodyStmt>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParserBodyStmtKind {
    Return {
        value: Option<Vec<ExprToken>>,
    },
    LocalVar {
        required_by: Option<ParserSymbolPath>,
        name: String,
        value_type: Option<ParserSymbolPath>,
        def_val: Vec<ExprToken>,
    },
    Assign {
        target: ParserSymbolPath,
        value: Vec<ExprToken>,
    },
    FuncCall {
        path: ParserSymbolPath,
        args: Vec<ParserFuncCallArg>,
    },
    If {
        main: ParserIfCond,
        else_ifs: Vec<ParserIfCond>,
        else_body: Vec<ParserBodyStmt>,
    },
    Block {
        statements: Vec<ParserBodyStmt>,
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
pub struct ParserIfCond {
    pub condition: Vec<ExprToken>,
    pub body: Vec<ParserBodyStmt>,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprToken {
    pub range: Range,
    pub kind: ExprTokenKind,
}

impl ExprToken {
    pub fn lparen(range: Range) -> Self {
        Self {
            range,
            kind: ExprTokenKind::LParen,
        }
    }

    pub fn rparen(range: Range) -> Self {
        Self {
            range,
            kind: ExprTokenKind::RParen,
        }
    }
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

// pub type ParserSymbolPath = Vec<ParserSymbolPathComponent>;
#[derive(Debug, PartialEq, Clone)]
pub struct ParserSymbolPath {
    pub path: Vec<ParserSymbolPathComponent>,
}

impl ParserSymbolPath {
    pub fn new(path: Vec<ParserSymbolPathComponent>) -> Self {
        Self { path }
    }
}

impl Display for ParserSymbolPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.path
                .iter()
                .map(|p| p.symbol.clone())
                .collect::<Vec<_>>()
                .join(".")
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserSymbolPathComponent {
    pub range: Range,
    pub symbol: String,
}
