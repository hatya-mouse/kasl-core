//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::ast::{Range, symbol_table::NoTypeFuncCallArg};

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct UnresolvedExpr {
    pub kind: UnresolvedExprKind,
    pub range: Range,
}

impl UnresolvedExpr {
    pub fn new(kind: UnresolvedExprKind, range: Range) -> Self {
        Self { kind, range }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum UnresolvedExprKind {
    IntLiteral(u32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    InfixOp {
        symbol: String,
        lhs_expr: Box<UnresolvedExpr>,
        rhs_expr: Box<UnresolvedExpr>,
    },
    PrefixOp {
        symbol: String,
        operand: Box<UnresolvedExpr>,
    },
    PostfixOp {
        symbol: String,
        operand: Box<UnresolvedExpr>,
    },
    Chain {
        lhs: Option<Box<UnresolvedExpr>>,
        elements: Vec<UnresolvedChainElement>,
    },
    Subscript {
        lhs: Box<UnresolvedExpr>,
        index: Box<UnresolvedExpr>,
    },
    ArraySpread {
        value: Box<UnresolvedExpr>,
        count: Box<UnresolvedExpr>,
    },
    ArrayList(Vec<UnresolvedExpr>),
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum UnresolvedChainElement {
    Identifier {
        name: String,
        range: Range,
    },
    FuncCall {
        name: String,
        args: Vec<NoTypeFuncCallArg>,
        range: Range,
    },
}
