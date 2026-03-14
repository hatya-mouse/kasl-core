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
mod chain_translator;
mod func_call_translator;
mod ident_translator;
mod literal_translator;
mod op_call_translator;
mod struct_init_translator;

use crate::{
    Expr, ExprKind, backend::func_translator::FuncTranslator, type_registry::ResolvedType,
};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn translate_expr(&mut self, expr: &Expr<ResolvedType>) -> Option<ir::Value> {
        match &expr.kind {
            ExprKind::IntLiteral(val) => Some(self.translate_int_literal(*val)),
            ExprKind::FloatLiteral(val) => Some(self.translate_float_literal(*val)),
            ExprKind::BoolLiteral(val) => Some(self.translate_bool_literal(*val)),
            ExprKind::InfixOp {
                operator, lhs, rhs, ..
            } => self.translate_infix_op_expr(
                &operator.unwrap(),
                lhs.as_ref().unwrap(),
                rhs.as_ref().unwrap(),
            ),
            ExprKind::PrefixOp {
                operator, operand, ..
            } => self.translate_prefix_op_expr(&operator.unwrap(), operand.as_ref().unwrap()),
            ExprKind::PostfixOp {
                operator, operand, ..
            } => self.translate_postfix_op_expr(&operator.unwrap(), operand.as_ref().unwrap()),
            ExprKind::Identifier { id, .. } => Some(self.translate_identifier(&id.unwrap())),
            ExprKind::FuncCall { id, args, .. } => {
                self.translate_func_call_expr(&id.unwrap(), args.as_ref().unwrap())
            }
            ExprKind::StructInit { id, .. } => self.translate_struct_init(id),
            ExprKind::Chain { lhs, access } => self.translate_chain(lhs, access, &expr.value_type),
            ExprKind::StaticFuncCall { id, args, .. } => self.translate_func_call_expr(&id, &args),
            ExprKind::BuiltinFuncCall { id, args, .. } => {
                self.translate_builtin_func_call(&id, &args)
            }
        }
    }
}
