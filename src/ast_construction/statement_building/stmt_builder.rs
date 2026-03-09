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
    FunctionID, ParserScopeStmt, ParserScopeStmtKind, ScopeID, Statement,
    statement_building::StatementBuilder,
};

impl StatementBuilder<'_> {
    pub fn build_stmt_for_func(&mut self, func_id: FunctionID) {
        if let Some(body) = self.func_body_map.get_body(&func_id) {
            // Get the scope ID of the function
            let Some(scope_id) = self
                .func_ctx
                .get_func_mut(&func_id)
                .map(|func| func.block.get_scope_id())
            else {
                return;
            };

            let mut resolved_body = Vec::new();
            for stmt in body {
                // Build the body one by one
                let Some(resolved_stmt) = self.build_stmt(stmt, scope_id) else {
                    continue;
                };
                resolved_body.push(resolved_stmt);
            }

            // Store the resolved body in the function
            let Some(func) = self.func_ctx.get_func_mut(&func_id) else {
                return;
            };
            func.block.set_stmt(resolved_body);
        }
    }

    pub fn build_stmt(&mut self, stmt: &ParserScopeStmt, scope_id: ScopeID) -> Option<Statement> {
        match &stmt.kind {
            ParserScopeStmtKind::Block { statements } => self.build_block(statements, scope_id),
            ParserScopeStmtKind::LocalVar {
                name,
                value_type,
                def_val,
            } => {}
            ParserScopeStmtKind::LocalConst {
                name,
                value_type,
                def_val,
            } => {}
            ParserScopeStmtKind::Assign { target, value } => {}
            ParserScopeStmtKind::FuncCall { path, args } => {}
            ParserScopeStmtKind::If {
                main,
                else_ifs,
                else_body,
            } => {}
            ParserScopeStmtKind::Return { value } => {}
        }
    }
}
