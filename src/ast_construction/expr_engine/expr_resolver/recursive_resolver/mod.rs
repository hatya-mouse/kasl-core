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

mod chain_resolver;
mod func_call_resolver;
mod identifier_resolver;
mod literal_resolver;
mod operator_resolver;

use crate::{
    Expr, ExprKind,
    error::Ph,
    expr_engine::ExpressionResolver,
    symbol_table::{UnresolvedExpr, UnresolvedExprKind},
};

impl ExpressionResolver<'_> {
    pub fn resolve_recursively(&mut self, expr: UnresolvedExpr) -> Option<Expr> {
        match expr.kind {
            UnresolvedExprKind::IntLiteral(value) => self.resolve_int_literal(value, expr.range),
            UnresolvedExprKind::FloatLiteral(value) => {
                self.resolve_float_literal(value, expr.range)
            }
            UnresolvedExprKind::BoolLiteral(value) => self.resolve_bool_literal(value, expr.range),

            UnresolvedExprKind::InfixOp {
                symbol,
                lhs_expr,
                rhs_expr,
            } => self.resolve_infix_op(symbol, *lhs_expr, *rhs_expr, expr.range),

            UnresolvedExprKind::PrefixOp { symbol, operand } => {
                self.resolve_prefix_op(symbol, *operand_expr, expr.range)
            }

            UnresolvedExprKind::PostfixOp { symbol, operand } => {
                self.resolve_postfix_op(symbol, *operand_expr, expr.range)
            }

            UnresolvedExprKind::Chain { lhs, elements } => {
                self.resolve_chain(*lhs, elements, expr.range)
            }

            _ => {
                self.ec.comp_bug(
                    expr.range,
                    Ph::ExprEngine,
                    "Received expression which should not exist at this point",
                );
                None
            }
        }
    }
}
