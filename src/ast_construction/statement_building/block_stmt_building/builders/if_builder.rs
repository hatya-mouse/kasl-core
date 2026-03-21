use crate::{
    IfArm, ParserIfArm, ParserScopeStmt, Range, Statement,
    error::Ph,
    expr_engine::resolve_expr,
    statement_building::BlockStmtBuilder,
    type_registry::{PrimitiveType, ResolvedType},
};

impl BlockStmtBuilder<'_> {
    pub fn build_if_stmt(
        &mut self,
        main: &ParserIfArm,
        else_ifs: &[ParserIfArm],
        else_body: Option<&Vec<ParserScopeStmt>>,
        else_range: Option<Range>,
    ) -> Option<Statement> {
        // Build the arms
        let main_arm = self.build_if_arm(main)?;
        let else_ifs = else_ifs
            .iter()
            .map(|arm| self.build_if_arm(arm))
            .collect::<Option<Vec<_>>>()?;
        // Build the else block
        // None is allowed because the else block is optional
        let else_block = else_body
            .map(|arm| self.build_scope_block(arm, self.scope_id, else_range.unwrap_or_default()));

        // Return the constructed if statement
        Some(Statement::If {
            main: main_arm,
            else_ifs,
            else_block,
        })
    }

    fn build_if_arm(&mut self, arm: &ParserIfArm) -> Option<IfArm> {
        // Resolve the condition expression and verify it has a bool type
        let condition = resolve_expr(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            self.scope_id,
            self.namespace_id,
            &arm.condition,
        )?;
        if condition.value_type != ResolvedType::Primitive(PrimitiveType::Bool) {
            self.ec.non_bool_type_for_condition(
                arm.range,
                Ph::StatementBuilding,
                self.prog_ctx
                    .type_registry
                    .format_type(&condition.value_type),
            );
            return None;
        }

        // Create a block for the arm's body
        let block = self.build_scope_block(&arm.body, self.scope_id, arm.range);
        Some(IfArm { condition, block })
    }
}
