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

use crate::{
    ast::{Expr, ExprKind},
    backend::func_translator::FuncTranslator,
};
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
