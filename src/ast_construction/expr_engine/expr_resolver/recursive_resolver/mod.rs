mod array_list_resolver;
mod array_spread_resolver;
mod chain_resolver;
mod literal_resolver;
mod operator_resolver;
mod subscript_resolver;

use crate::{
    Expr,
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
                self.resolve_prefix_op(symbol, *operand, expr.range)
            }

            UnresolvedExprKind::PostfixOp { symbol, operand } => {
                self.resolve_postfix_op(symbol, *operand, expr.range)
            }

            UnresolvedExprKind::Chain { lhs, elements } => {
                self.resolve_chain(lhs.map(|lhs| *lhs), elements, expr.range)
            }

            UnresolvedExprKind::Subscript { lhs, index } => {
                self.resolve_subscript(*lhs, *index, expr.range)
            }

            UnresolvedExprKind::ArraySpread { value, count } => {
                self.resolve_array_spread(*value, *count, expr.range)
            }

            UnresolvedExprKind::ArrayList(items) => self.resolve_array_list(items, expr.range),
        }
    }
}
