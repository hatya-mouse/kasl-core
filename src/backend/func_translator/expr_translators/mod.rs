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

mod builtin_call_translator;
mod func_call_translator;
mod ident_translator;
mod instance_call_translator;
mod literal_translator;
mod op_call_translator;
mod struct_field_translator;
mod struct_init_translator;

use crate::{Expr, ExprKind, backend::func_translator::FuncTranslator};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn translate_expr(&mut self, expr: &Expr) -> ir::Value {
        match &expr.kind {
            ExprKind::IntLiteral(val) => self.translate_int_literal(*val),
            ExprKind::FloatLiteral(val) => self.translate_float_literal(*val),
            ExprKind::BoolLiteral(val) => self.translate_bool_literal(*val),
            ExprKind::InfixOp {
                operator, lhs, rhs, ..
            } => self.translate_infix_op_expr(operator, lhs, rhs),
            ExprKind::PrefixOp {
                operator, operand, ..
            } => self.translate_prefix_op_expr(operator, operand),
            ExprKind::PostfixOp {
                operator, operand, ..
            } => self.translate_postfix_op_expr(operator, operand),
            ExprKind::Identifier { id, .. } => self.translate_identifier(id),
            ExprKind::StructField { lhs, offset } => {
                self.translate_struct_field_expr(lhs, &expr.value_type, *offset)
            }
            ExprKind::StructInit { id, .. } => self.translate_struct_init(id),
            ExprKind::StaticFuncCall { id, args, .. } => self.translate_func_call_expr(id, args),
            ExprKind::InstanceFuncCall { lhs, id, args, .. } => {
                self.translate_instance_call_expr(lhs, id, args)
            }
            ExprKind::FuncCall { id, args, .. } => self.translate_func_call_expr(id, args),
            ExprKind::BuiltinFuncCall { id, args, .. } => {
                self.translate_builtin_func_call(id, args)
            }
        }
    }
}
