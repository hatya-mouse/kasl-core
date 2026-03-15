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

mod unresolved_expr;

pub use unresolved_expr::{UnresolvedChainElement, UnresolvedExpr, UnresolvedExprKind};

use crate::{
    FuncCallArg, FunctionID, OperatorID, Range, StructID, VariableID, builtin::BuiltinFuncID,
    symbol_table::NoTypeFuncCallArg, type_registry::ResolvedType,
};

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct Expr {
    pub kind: ExprKind,
    pub value_type: ResolvedType,
    pub range: Range,
}

impl Expr {
    pub fn new(kind: ExprKind, value_type: ResolvedType, range: Range) -> Self {
        Self {
            kind,
            value_type,
            range,
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum ExprKind {
    IntLiteral(i32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    InfixOp {
        operator: OperatorID,
        lhs: Box<FuncCallArg>,
        rhs: Box<FuncCallArg>,
    },
    PrefixOp {
        operator: OperatorID,
        operand: Box<FuncCallArg>,
    },
    PostfixOp {
        operator: OperatorID,
        operand: Box<FuncCallArg>,
    },
    Identifier {
        name: String,
        id: VariableID,
    },
    FuncCall {
        id: FunctionID,
        args: Vec<FuncCallArg>,
    },
    StructInit {
        name: String,
        id: StructID,
    },
    Chain {
        lhs: Box<Expr>,
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
        args: Vec<Expr>,
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
