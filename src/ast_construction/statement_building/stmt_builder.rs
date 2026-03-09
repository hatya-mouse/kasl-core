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

use crate::{FunctionID, ParserScopeStmtKind, statement_building::StatementBuilder};

impl StatementBuilder<'_> {
    pub fn build_stmt_for_func(&mut self, func_id: FunctionID) {
        if let Some(body) = self.func_body_map.get_body(&func_id) {
            for stmt in body {
                match &stmt.kind {
                    ParserScopeStmtKind::Block { statements } => {}
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
    }
}
