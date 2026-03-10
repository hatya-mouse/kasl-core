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
    ParserScopeStmt, ParserScopeStmtKind, ScopeID, Statement, statement_building::FuncStmtBuilder,
};

impl FuncStmtBuilder<'_> {
    pub fn build_func_body(&mut self) {
        if let Some(body) = self.func_body_map.get_body(&self.func_id) {
            // Generate a new Scope for the function
            let global_scope_id = self.scope_registry.get_global_scope_id();
            let resolved_body = self.build_scope_block(body, global_scope_id);

            // Store the resolved body in the function
            let Some(func) = self.func_ctx.get_func_mut(&self.func_id) else {
                return;
            };
            func.set_block(resolved_body);
        }
    }

    pub fn build_stmt(&mut self, stmt: &ParserScopeStmt, scope_id: ScopeID) -> Option<Statement> {
        match &stmt.kind {
            ParserScopeStmtKind::Block { statements } => {
                self.build_block_stmt(statements, scope_id)
            }
            ParserScopeStmtKind::LocalVar {
                name,
                value_type,
                def_val,
            } => self.build_local_var(name, value_type, def_val, scope_id, stmt.range),
            ParserScopeStmtKind::LocalConst {
                name,
                value_type,
                def_val,
            } => self.build_local_const(name, value_type, def_val, scope_id, stmt.range),
            ParserScopeStmtKind::Assign { target, value } => {
                self.build_assign(target, value, scope_id, stmt.range)
            }
            ParserScopeStmtKind::Expression { expr } => self.build_expr_stmt(expr, scope_id),
            ParserScopeStmtKind::If {
                main,
                else_ifs,
                else_body,
            } => self.build_if_stmt(main, else_ifs, else_body.as_ref(), scope_id),
            ParserScopeStmtKind::Return { value } => {
                self.build_return_stmt(value.as_ref(), scope_id, stmt.range)
            }
        }
    }
}
