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
    ast::Statement,
    ast_construction::statement_building::BlockStmtBuilder,
    parser::{ParserScopeStmt, ParserScopeStmtKind},
};

impl BlockStmtBuilder<'_> {
    pub fn build_stmt(&mut self, stmt: &ParserScopeStmt) -> Option<Statement> {
        match &stmt.kind {
            ParserScopeStmtKind::Block { statements } => {
                self.build_block_stmt(statements, stmt.range)
            }
            ParserScopeStmtKind::LocalVar {
                name,
                value_type,
                def_val,
            } => self.build_local_var(name, value_type, def_val, stmt.range),
            ParserScopeStmtKind::LocalConst {
                name,
                value_type,
                def_val,
            } => self.build_local_const(name, value_type, def_val, stmt.range),
            ParserScopeStmtKind::Assign { target, value } => {
                self.build_assign(target, value, stmt.range)
            }
            ParserScopeStmtKind::Expression { expr } => self.build_expr_stmt(expr),
            ParserScopeStmtKind::If {
                main,
                else_ifs,
                else_body,
                else_range,
            } => self.build_if_stmt(main, else_ifs, else_body.as_ref(), *else_range),
            ParserScopeStmtKind::Return { value } => {
                self.build_return_stmt(value.as_ref(), stmt.range)
            }
            ParserScopeStmtKind::Loop { count, body } => {
                self.build_loop_stmt(count, body, stmt.range)
            }
        }
    }
}
