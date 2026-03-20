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

use crate::{
    InfixOperatorProperties, PostfixOperatorProperties, PrefixOperatorProperties, Range,
    SymbolPath, namespace_registry::ImportPath, symbol_table::UnresolvedExpr,
};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct ParserProgram {
    pub statements: Vec<ParserDeclStmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserDeclStmt {
    pub range: Range,
    pub kind: ParserDeclStmtKind,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct ParserScopeStmt {
    pub range: Range,
    pub kind: ParserScopeStmtKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParserDeclStmtKind {
    Import {
        path: ImportPath,
    },
    FuncDecl {
        is_static: bool,
        name: String,
        params: Vec<ParserFuncParam>,
        return_type: Option<ParserTypeName>,
        body: Vec<ParserScopeStmt>,
    },
    Input {
        name: String,
        value_type: Option<ParserTypeName>,
        def_val: Vec<ExprToken>,
        attrs: Vec<ParserInputAttribute>,
    },
    Output {
        name: String,
        value_type: Option<ParserTypeName>,
        def_val: Vec<ExprToken>,
    },
    StateVar {
        name: String,
        value_type: Option<ParserTypeName>,
        def_val: Vec<ExprToken>,
    },
    GlobalConst {
        name: String,
        value_type: Option<ParserTypeName>,
        def_val: Vec<ExprToken>,
    },
    StructField {
        name: String,
        value_type: Option<ParserTypeName>,
        def_val: Vec<ExprToken>,
    },
    StructDecl {
        name: String,
        body: Vec<ParserDeclStmt>,
    },
    InfixDefine {
        symbol: String,
        props: InfixOperatorProperties,
    },
    PrefixDefine {
        symbol: String,
        props: PrefixOperatorProperties,
    },
    PostfixDefine {
        symbol: String,
        props: PostfixOperatorProperties,
    },
    OperatorFunc {
        op_type: ParserOperatorType,
        symbol: String,
        params: Vec<ParserFuncParam>,
        return_type: ParserTypeName,
        body: Vec<ParserScopeStmt>,
    },
}

impl Display for ParserDeclStmtKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserDeclStmtKind::Import { .. } => write!(f, "import"),
            ParserDeclStmtKind::FuncDecl { .. } => write!(f, "func"),
            ParserDeclStmtKind::Input { .. } => write!(f, "input"),
            ParserDeclStmtKind::Output { .. } => write!(f, "output"),
            ParserDeclStmtKind::StateVar { .. } => write!(f, "state"),
            ParserDeclStmtKind::GlobalConst { .. } => write!(f, "let"),
            ParserDeclStmtKind::StructField { .. } => write!(f, "var"),
            ParserDeclStmtKind::StructDecl { .. } => write!(f, "struct"),
            ParserDeclStmtKind::InfixDefine { .. } => write!(f, "infix"),
            ParserDeclStmtKind::PrefixDefine { .. } => write!(f, "prefix"),
            ParserDeclStmtKind::PostfixDefine { .. } => write!(f, "postfix"),
            ParserDeclStmtKind::OperatorFunc { .. } => write!(f, "func"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum ParserScopeStmtKind {
    Block {
        statements: Vec<ParserScopeStmt>,
    },
    LocalVar {
        name: String,
        value_type: Option<ParserTypeName>,
        def_val: Vec<ExprToken>,
    },
    LocalConst {
        name: String,
        value_type: Option<ParserTypeName>,
        def_val: Vec<ExprToken>,
    },
    Assign {
        target: Vec<ExprToken>,
        value: Vec<ExprToken>,
    },
    Expression {
        expr: Vec<ExprToken>,
    },
    If {
        main: ParserIfArm,
        else_ifs: Vec<ParserIfArm>,
        else_body: Option<Vec<ParserScopeStmt>>,
        else_range: Option<Range>,
    },
    Return {
        value: Option<Vec<ExprToken>>,
    },
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum ParserOperatorType {
    Infix,
    Prefix,
    Postfix,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserInputAttribute {
    pub name: String,
    pub args: Vec<Vec<ExprToken>>,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserFuncParam {
    pub label: Option<String>,
    pub name: String,
    pub value_type: Option<ParserTypeName>,
    pub def_val: Option<Vec<ExprToken>>,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct ParserIfArm {
    pub condition: Vec<ExprToken>,
    pub body: Vec<ParserScopeStmt>,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum ParserTypeName {
    SymbolPath(SymbolPath),
    Array(Box<ParserTypeName>, u32),
}

impl Display for ParserTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserTypeName::SymbolPath(path) => write!(f, "{}", &path.to_string()),
            ParserTypeName::Array(item_type, count) => {
                write!(f, "[{}; {}]", item_type.to_string(), count)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct ParserFuncCallArg {
    pub label: Option<String>,
    pub value: Vec<ExprToken>,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct ExprToken {
    pub range: Range,
    pub kind: ExprTokenKind,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum ExprTokenKind {
    IntLiteral(u32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    Operator(String),
    Identifier(String),
    FuncCall {
        name: String,
        args: Vec<ParserFuncCallArg>,
    },
    Dot,
    Parenthesized(Vec<ExprToken>),
    Subscription {
        array: Vec<ExprToken>,
        index: Vec<ExprToken>,
    },
    /// An unresolved expression which is used only in the ExprEngine.
    UnresolvedExpr(UnresolvedExpr),
}
