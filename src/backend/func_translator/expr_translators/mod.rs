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

mod func_call_translator;
mod infix_op_translator;
mod literal_translator;

use crate::{
    Expr, ExprKind, backend::func_translator::FuncTranslator, type_registry::ResolvedType,
};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn translate_expr(&mut self, expr: &Expr<ResolvedType>) -> ir::Value {
        match &expr.kind {
            ExprKind::IntLiteral(val) => self.translate_int_literal(*val),
            ExprKind::FloatLiteral(val) => self.translate_float_literal(*val),
            ExprKind::BoolLiteral(val) => self.translate_bool_literal(*val),
            ExprKind::InfixOp {
                symbol,
                operator,
                lhs,
                rhs,
            } => self.translate_infix_op_expr(*val),
        }
    }
}
