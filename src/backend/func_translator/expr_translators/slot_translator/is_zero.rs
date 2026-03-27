use crate::{Expr, ExprKind, backend::func_translator::FuncTranslator};

impl FuncTranslator<'_> {
    /// Checks if the passed expression is a zero literal.
    pub(super) fn is_zero(&mut self, expr: &Expr) -> bool {
        match &expr.kind {
            ExprKind::BoolLiteral(value) => !value,
            ExprKind::FloatLiteral(value) => value == &0.0,
            ExprKind::IntLiteral(value) => value == &0,
            _ => false,
        }
    }
}
