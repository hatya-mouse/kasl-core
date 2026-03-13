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

use crate::{
    ParserScopeStmt, ScopeID, Statement, statement_building::BlockStmtBuilder,
    type_registry::ResolvedType,
};

impl BlockStmtBuilder<'_> {
    /// Builds a block statement from a list of statements.
    pub fn build_block_stmt(
        &mut self,
        statements: &[ParserScopeStmt],
        parent_scope_id: ScopeID,
        expected_return_type: ResolvedType,
    ) -> Option<Statement> {
        let block = self.build_scope_block(statements, parent_scope_id, expected_return_type);

        // Check if the child block has a return statement
        let does_child_have_return = self.scope_guarantees_return(block.scope_id);
        self.scope_has_return
            .insert(parent_scope_id, does_child_have_return);

        Some(Statement::Block { block })
    }
}
