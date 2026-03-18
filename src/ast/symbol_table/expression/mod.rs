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

mod l_value;
mod unresolved_expr;

pub use l_value::LValue;
pub use unresolved_expr::{UnresolvedChainElement, UnresolvedExpr, UnresolvedExprKind};

use crate::{
    FunctionID, OperatorID, Range, StructID, VariableID, builtin::BuiltinFuncID,
    type_registry::ResolvedType,
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
        lhs: VariableID,
        rhs: VariableID,
    },
    PrefixOp {
        operator: OperatorID,
        operand: VariableID,
    },
    PostfixOp {
        operator: OperatorID,
        operand: VariableID,
    },
    Identifier {
        id: VariableID,
    },
    StructField {
        lhs: Box<Expr>,
        offset: i32,
    },
    StructInit {
        id: StructID,
    },
    StaticFuncCall {
        id: FunctionID,
        args: Vec<VariableID>,
    },
    InstanceFuncCall {
        id: FunctionID,
        args: Vec<VariableID>,
    },
    FuncCall {
        id: FunctionID,
        args: Vec<VariableID>,
    },
    BuiltinFuncCall {
        id: BuiltinFuncID,
        args: Vec<Expr>,
    },
}
