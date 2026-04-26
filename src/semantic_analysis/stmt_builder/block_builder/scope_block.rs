//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::{
    ast_nodes::{Range, ScopeID, Statement, symbol_table::Block},
    parser::ParserScopeStmt,
    semantic_analysis::stmt_builder::BlockStmtBuilder,
};

impl BlockStmtBuilder<'_> {
    pub fn build_statements(&mut self, statements: &[ParserScopeStmt]) -> Vec<Statement> {
        let mut body = Vec::new();
        // Build each statement in the block scope
        for stmt in statements {
            let Some(resolved_stmt) = self.build_stmt(stmt) else {
                continue;
            };

            // If the statement is return or break statement, return the body and stop building
            let is_return = matches!(&resolved_stmt, Statement::Return { .. });

            // Push the resolved statement to the body
            body.push(resolved_stmt);

            if is_return {
                return body;
            }
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
            self.flow_graph_builder,
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
