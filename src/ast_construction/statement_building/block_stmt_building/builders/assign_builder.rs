use crate::{
    ExprToken, Range, Statement,
    error::Ph,
    expr_engine::{LValueResolver, resolve_expr},
    statement_building::BlockStmtBuilder,
};

impl BlockStmtBuilder<'_> {
    pub fn build_assign(
        &mut self,
        target: &[ExprToken],
        value: &[ExprToken],
        stmt_range: Range,
    ) -> Option<Statement> {
        // Resolve the target variable
        let mut l_value_resolver = LValueResolver::new(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            self.scope_id,
            self.namespace_id,
        );

        // Error will be thrown by the LValueResolver so no need to check for None
        let target_l_value = l_value_resolver.resolve_l_value(target)?;

        // Resolve the expression
        let resolved_value = resolve_expr(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            self.scope_id,
            self.namespace_id,
            value,
        )?;

        // Check if the target and value types match
        if target_l_value.value_type != resolved_value.value_type {
            self.ec.assign_type_mismatch(
                stmt_range,
                Ph::StatementBuilding,
                self.prog_ctx
                    .type_registry
                    .format_type(&target_l_value.value_type),
                self.prog_ctx
                    .type_registry
                    .format_type(&resolved_value.value_type),
            );
            return None;
        }

        Some(Statement::Assign {
            target: target_l_value,
            value: resolved_value,
        })
    }
}
