use crate::{
    ExprToken, Statement, expr_engine::resolve_expr, statement_building::BlockStmtBuilder,
};

impl BlockStmtBuilder<'_> {
    pub fn build_expr_stmt(&mut self, expr: &[ExprToken]) -> Option<Statement> {
        let expr = resolve_expr(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            self.scope_id,
            self.namespace_id,
            expr,
        )?;
        Some(Statement::Expression { expr })
    }
}
