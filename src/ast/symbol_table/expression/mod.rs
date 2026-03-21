mod l_value;
mod unresolved_expr;

pub use l_value::{LValue, LValueKind};
pub use unresolved_expr::{UnresolvedChainElement, UnresolvedExpr, UnresolvedExprKind};

use crate::{
    FuncCallArg, FunctionID, OperatorID, Range, StructID, VariableID, builtin::BuiltinFuncID,
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
    IntLiteral(u32),
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
    Identifier(VariableID),
    StructField {
        lhs: Box<Expr>,
        offset: i32,
    },
    StructInit {
        id: StructID,
    },
    StaticFuncCall {
        id: FunctionID,
        args: Vec<FuncCallArg>,
    },
    InstanceFuncCall {
        id: FunctionID,
        args: Vec<FuncCallArg>,
    },
    FuncCall {
        id: FunctionID,
        args: Vec<FuncCallArg>,
    },
    BuiltinFuncCall {
        id: BuiltinFuncID,
        args: Vec<Expr>,
    },
    Subscript {
        lhs: Box<Expr>,
        index: Box<Expr>,
    },
    ArraySpread {
        value: Box<Expr>,
        count: u32,
    },
    ArrayList(Vec<Expr>),
}
