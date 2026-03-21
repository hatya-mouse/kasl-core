use crate::{
    ExprToken, ParserScopeStmt, Range, Statement,
    common_utils::get_constant_int,
    error::Ph,
    expr_engine::resolve_expr,
    statement_building::BlockStmtBuilder,
    type_registry::{PrimitiveType, ResolvedType},
};

impl BlockStmtBuilder<'_> {
    pub fn build_loop_stmt(
        &mut self,
        count: &[ExprToken],
        body: &[ParserScopeStmt],
        decl_range: Range,
    ) -> Option<Statement> {
        // Parse the count
        let count_expr = resolve_expr(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            self.scope_id,
            self.namespace_id,
            count,
        )?;

        // Check if the count has integer type
        if !matches!(
            count_expr.value_type,
            ResolvedType::Primitive(PrimitiveType::Int)
        ) {
            self.ec.non_integer_for_loop_count(
                count_expr.range,
                Ph::ExprEngine,
                self.prog_ctx
                    .type_registry
                    .format_type(&count_expr.value_type),
            );
            return None;
        }

        // Verify the count expression and get the integer loop count
        if let Some(loop_count) = get_constant_int(&self.prog_ctx.scope_registry, &count_expr) {
            // Build the body and return the new loop statement
            let loop_block = self.build_scope_block(body, self.scope_id, decl_range);
            Some(Statement::Loop {
                count: loop_count,
                body: loop_block,
            })
        } else {
            self.ec
                .non_constant_for_loop_count(count_expr.range, Ph::ExprEngine);
            None
        }
    }
}
