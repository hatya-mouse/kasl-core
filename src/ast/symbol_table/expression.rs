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
    FuncCallArg, FunctionID, OperatorID, Range, StructID, VariableID,
    symbol_table::NoTypeFuncCallArg, type_registry::ResolvedType,
};

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct Expr<T> {
    pub kind: ExprKind<T>,
    pub value_type: T,
    pub range: Range,
}

impl<T> Expr<T> {
    pub fn new(kind: ExprKind<T>, value_type: T, range: Range) -> Self {
        Self {
            kind,
            value_type,
            range,
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum ExprKind<T> {
    IntLiteral(i32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    InfixOp {
        symbol: String,
        operator: Option<OperatorID>,
        lhs_expr: Box<Expr<T>>,
        lhs: Option<Box<FuncCallArg>>,
        rhs_expr: Box<Expr<T>>,
        rhs: Option<Box<FuncCallArg>>,
    },
    PrefixOp {
        symbol: String,
        operator: Option<OperatorID>,
        operand_expr: Box<Expr<T>>,
        operand: Option<Box<FuncCallArg>>,
    },
    PostfixOp {
        symbol: String,
        operator: Option<OperatorID>,
        operand_expr: Box<Expr<T>>,
        operand: Option<Box<FuncCallArg>>,
    },
    Identifier {
        name: String,
        id: Option<VariableID>,
    },
    FuncCall {
        name: String,
        id: Option<FunctionID>,
        no_type_args: Vec<NoTypeFuncCallArg>,
        args: Option<Vec<FuncCallArg>>,
    },
    StructInit {
        name: String,
        id: StructID,
    },
    Chain {
        lhs: Box<Expr<T>>,
        access: MemberAccess,
    },
    StaticFuncCall {
        name: String,
        id: FunctionID,
        args: Vec<FuncCallArg>,
    },
    BuiltinFuncCall {
        name: String,
        id: BuiltinFuncID,
        args: Option<Vec<FuncCallArg>>,
    },
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum MemberAccess {
    Access {
        name: String,
        offset: Option<i32>,
    },
    FuncCall {
        name: String,
        id: Option<FunctionID>,
        no_type_args: Vec<NoTypeFuncCallArg>,
        args: Option<Vec<FuncCallArg>>,
    },
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct LValue {
    pub var_id: VariableID,
    pub offset: i32,
    pub value_type: ResolvedType,
    pub is_field: bool,
}

pub enum ResolvedChainLHS {
    Expr(Expr<ResolvedType>),
    Type(ResolvedType),
    Builtin,
}
