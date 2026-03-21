use crate::{
    ParserScopeStmt, Range, ScopeID, Statement, statement_building::BlockStmtBuilder,
    symbol_table::Block,
};

impl BlockStmtBuilder<'_> {
    pub fn build_statements(&mut self, statements: &[ParserScopeStmt]) -> Vec<Statement> {
        let mut body = Vec::new();
        // Build each statement in the block scope
        for stmt in statements {
            let Some(resolved_stmt) = self.build_stmt(stmt) else {
                continue;
            };
            body.push(resolved_stmt);
        }
        body
    }

    pub fn build_scope_block(
        &mut self,
        statements: &[ParserScopeStmt],
        parent_scope_id: ScopeID,
        decl_range: Range,
    ) -> Block {
        // Create a new scope for the block
        let block_scope_id = self
            .prog_ctx
            .scope_registry
            .create_scope(Some(parent_scope_id), decl_range);

        // Create a block statement builder
        let mut block_builder = BlockStmtBuilder::new(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            block_scope_id,
            self.namespace_id,
            self.expected_return_type,
        );
        // Build the statements in the scope
        let body = block_builder.build_statements(statements);

        // Add an edge from the parent scope to the block scope
        self.comp_data
            .scope_graph
            .add_edge(parent_scope_id, block_scope_id);

        // Create a block for the resolved statements
        let mut block = Block::new(block_scope_id);
        block.set_stmt(body);
        block
    }
}
