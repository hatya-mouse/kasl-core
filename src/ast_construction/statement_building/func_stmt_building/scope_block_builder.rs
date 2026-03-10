//
// © 2025-2026 Shuntaro Kasatani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use crate::{ParserScopeStmt, ScopeID, statement_building::FuncStmtBuilder, symbol_table::Block};

impl FuncStmtBuilder<'_> {
    pub fn build_scope_block(
        &mut self,
        statements: &[ParserScopeStmt],
        parent_scope_id: ScopeID,
    ) -> Block {
        let mut body = Vec::new();

        // Create a new scope for the block
        let block_scope_id = self.scope_registry.create_scope(Some(parent_scope_id));
        // Build each statement in the block scope
        for stmt in statements {
            let Some(resolved_stmt) = self.build_stmt(stmt, block_scope_id) else {
                continue;
            };
            body.push(resolved_stmt);
        }

        // Add an edge from the parent scope to the block scope
        self.scope_graph.add_edge(parent_scope_id, block_scope_id);

        // Create a block for the resolved statements
        let mut block = Block::new(block_scope_id);
        block.set_stmt(body);
        block
    }
}
