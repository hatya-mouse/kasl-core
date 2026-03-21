use crate::{
    ExprToken, Range, Statement, error::Ph, expr_engine::resolve_expr,
    statement_building::BlockStmtBuilder,
};

impl BlockStmtBuilder<'_> {
    pub fn build_return_stmt(
        &mut self,
        value: Option<&Vec<ExprToken>>,
        decl_range: Range,
    ) -> Option<Statement> {
        // The current scope has a return statement
        self.comp_data
            .scope_graph
            .set_has_return(self.scope_id, true);

        if self.expected_return_type.is_void() {
            if value.is_some() {
                // If the function doesn't require a return value but it's provided, throw and error
                self.ec
                    .return_value_for_no_return_func(decl_range, Ph::StatementBuilding);
                None
            } else {
                Some(Statement::Return { value: None })
            }
        } else if let Some(value) = value {
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

            // Check if the return type matches the expected return type
            // If the self.expected_return_type is None, resolved_value should be None as well
            if resolved_value.value_type != self.expected_return_type {
                self.ec.return_type_mismatch(
                    decl_range,
                    Ph::StatementBuilding,
                    self.prog_ctx
                        .type_registry
                        .format_type(&self.expected_return_type),
                    self.prog_ctx
                        .type_registry
                        .format_type(&resolved_value.value_type),
                );
                return None;
            }

            Some(Statement::Return {
                value: Some(resolved_value),
            })
        } else {
            self.ec.return_without_value_for_return_func(
                decl_range,
                Ph::StatementBuilding,
                self.prog_ctx
                    .type_registry
                    .format_type(&self.expected_return_type),
            );
            None
        }
    }
}
