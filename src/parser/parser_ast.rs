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

use crate::{InfixOperatorProperties, Range, SymbolPath};
use std::fmt::Display;

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
        is_static: bool,
        name: String,
        params: Vec<ParserFuncParam>,
        return_type: Option<SymbolPath>,
        body: Option<Vec<ParserBodyStmt>>,
    },
    Input {
        name: String,
        value_type: Option<SymbolPath>,
        def_val: Vec<ExprToken>,
        attrs: Vec<ParserInputAttribute>,
    },
    Output {
        name: String,
        value_type: Option<SymbolPath>,
        def_val: Vec<ExprToken>,
    },
    StateVar {
        name: String,
        value_type: Option<SymbolPath>,
        def_val: Vec<ExprToken>,
    },
    ScopeVar {
        name: String,
        value_type: Option<SymbolPath>,
        def_val: Vec<ExprToken>,
    },
    StructDecl {
        name: String,
        body: Vec<ParserTopLevelStmt>,
    },
    InfixDefine {
        symbol: String,
        infix_properties: InfixOperatorProperties,
    },
    OperatorFunc {
        op_type: ParserOperatorType,
        symbol: String,
        params: Vec<ParserFuncParam>,
        return_type: SymbolPath,
        body: Vec<ParserBodyStmt>,
    },
}

impl Display for ParserTopLevelStmtKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserTopLevelStmtKind::FuncDecl { .. } => write!(f, "func"),
            ParserTopLevelStmtKind::Input { .. } => write!(f, "input"),
            ParserTopLevelStmtKind::Output { .. } => write!(f, "output"),
            ParserTopLevelStmtKind::StateVar { .. } => write!(f, "state"),
            ParserTopLevelStmtKind::ScopeVar { .. } => write!(f, "var"),
            ParserTopLevelStmtKind::StructDecl { .. } => write!(f, "struct"),
            ParserTopLevelStmtKind::InfixDefine { .. } => write!(f, "infix"),
            ParserTopLevelStmtKind::OperatorFunc { .. } => write!(f, "func"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParserBodyStmtKind {
    Return {
        value: Option<Vec<ExprToken>>,
    },
    LocalVar {
        name: String,
        value_type: Option<SymbolPath>,
        def_val: Vec<ExprToken>,
    },
    Assign {
        target: SymbolPath,
        value: Vec<ExprToken>,
    },
    FuncCall {
        path: SymbolPath,
        args: Vec<ParserFuncCallArg>,
    },
    If {
        main: ParserIfArm,
        else_ifs: Vec<ParserIfArm>,
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
pub struct ParserFuncParam {
    pub label: Option<String>,
    pub name: String,
    pub value_type: Option<SymbolPath>,
    pub def_val: Option<Vec<ExprToken>>,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserIfArm {
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
    Identifier(SymbolPath),
    FuncCall {
        path: SymbolPath,
        args: Vec<ParserFuncCallArg>,
    },
    LParen,
    RParen,
}
