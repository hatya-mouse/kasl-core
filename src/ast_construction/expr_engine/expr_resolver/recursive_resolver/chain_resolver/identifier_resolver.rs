use crate::{Expr, ExprKind, Range, ScopeID, error::Ph, expr_engine::ExpressionResolver};

impl ExpressionResolver<'_> {
    pub fn resolve_identifier(
        &mut self,
        target_scope: ScopeID,
        name: &str,
        range: Range,
    ) -> Option<Expr> {
        // Look up the variable in the target scope
        let Some(var_id) = self.prog_ctx.scope_registry.get_var_id(target_scope, name) else {
            self.ec.var_not_found(range, Ph::ExprEngine, name);
            return None;
        };

        // Get a reference to the variable
        let scope_var = self.prog_ctx.scope_registry.get_var(&var_id)?;

        Some(Expr::new(
            ExprKind::Identifier(var_id),
            scope_var.value_type,
            range,
        ))
    }
}
