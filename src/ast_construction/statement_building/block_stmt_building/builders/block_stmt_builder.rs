use crate::{ParserScopeStmt, Range, Statement, statement_building::BlockStmtBuilder};

impl BlockStmtBuilder<'_> {
    /// Builds a block statement from a list of statements.
    pub fn build_block_stmt(
        &mut self,
        statements: &[ParserScopeStmt],
        decl_range: Range,
    ) -> Option<Statement> {
        let block = self.build_scope_block(statements, self.scope_id, decl_range);
        Some(Statement::Block { block })
    }
}
