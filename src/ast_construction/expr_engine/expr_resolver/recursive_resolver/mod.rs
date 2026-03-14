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
    Expr, ExprKind, error::Ph, expr_engine::ExpressionResolver, type_registry::ResolvedType,
};

impl ExpressionResolver<'_> {
    pub fn resolve_recursively(&mut self, expr: Expr<()>) -> Option<Expr<ResolvedType>> {
        match expr.kind {
            ExprKind::IntLiteral(value) => self.resolve_int_literal(value, expr.range),
            ExprKind::FloatLiteral(value) => self.resolve_float_literal(value, expr.range),
            ExprKind::BoolLiteral(value) => self.resolve_bool_literal(value, expr.range),

            ExprKind::InfixOp {
                symbol,
                lhs_expr,
                rhs_expr,
                ..
            } => self.resolve_infix_op(symbol, *lhs_expr, *rhs_expr, expr.range),

            ExprKind::PrefixOp {
                symbol,
                operand_expr,
                ..
            } => self.resolve_prefix_op(symbol, *operand_expr, expr.range),

            ExprKind::PostfixOp {
                symbol,
                operand_expr,
                ..
            } => self.resolve_postfix_op(symbol, *operand_expr, expr.range),

            ExprKind::Identifier { name, id: _ } => self.resolve_identifier(name, expr.range),

            ExprKind::FuncCall {
                name,
                id: _,
                no_type_args,
                args: _,
            } => self.resolve_func_call(name, no_type_args, expr.range),

            ExprKind::Chain { lhs, access } => self.resolve_chain(*lhs, access, expr.range),

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
