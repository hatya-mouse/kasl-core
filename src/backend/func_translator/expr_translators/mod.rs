mod array_literal;
mod builtin_call;
mod func_call;
mod ident;
mod instance_call;
mod literal;
mod op_call;
mod slot_translator;
mod struct_field;
mod struct_init;
mod subscript;

use crate::{Expr, ExprKind, backend::func_translator::FuncTranslator};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn translate_expr(&mut self, expr: &Expr) -> Option<ir::Value> {
        match &expr.kind {
            ExprKind::IntLiteral(val) => Some(self.translate_int_literal(*val)),
            ExprKind::FloatLiteral(val) => Some(self.translate_float_literal(*val)),
            ExprKind::BoolLiteral(val) => Some(self.translate_bool_literal(*val)),
            ExprKind::InfixOp {
                operator, lhs, rhs, ..
            } => Some(self.translate_infix_op_expr(operator, lhs, rhs)),
            ExprKind::PrefixOp {
                operator, operand, ..
            } => Some(self.translate_prefix_op_expr(operator, operand)),
            ExprKind::PostfixOp {
                operator, operand, ..
            } => Some(self.translate_postfix_op_expr(operator, operand)),
            ExprKind::Identifier(id) => Some(self.translate_identifier(id)),
            ExprKind::StructField { lhs, offset } => {
                Some(self.translate_struct_field_expr(lhs, &expr.value_type, *offset))
            }
            ExprKind::StructInit { id, .. } => Some(self.translate_struct_init(id)),
            ExprKind::StaticFuncCall { id, args, .. } => self.translate_func_call_expr(id, args),
            ExprKind::InstanceFuncCall { id, args, .. } => {
                self.translate_instance_call_expr(id, args)
            }
            ExprKind::FuncCall { id, args, .. } => self.translate_func_call_expr(id, args),
            ExprKind::BuiltinFuncCall { id, args, .. } => {
                Some(self.translate_builtin_func_call(id, args))
            }
            ExprKind::ArrayList(_) => Some(self.translate_array_literal(expr)),
            ExprKind::ArraySpread { .. } => Some(self.translate_array_literal(expr)),
            ExprKind::Subscript { lhs, index } => {
                Some(self.translate_subscript(&expr.value_type, lhs, index))
            }
        }
    }
}
