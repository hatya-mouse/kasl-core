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

use crate::{ParserDeclStmt, ParserDeclStmtKind, global_decl_collection::GlobalDeclCollector};

impl GlobalDeclCollector<'_> {
    pub fn process_stmt(&mut self, stmt: &ParserDeclStmt) {
        match &stmt.kind {
            ParserDeclStmtKind::Input {
                name,
                value_type,
                def_val,
                attrs,
            } => self.resolve_input(name, value_type, def_val, attrs, stmt.range),
            ParserDeclStmtKind::Output { .. } => self.resolve_output(stmt),
            ParserDeclStmtKind::StateVar { .. } => self.resolve_state_var(stmt),
            ParserDeclStmtKind::StructDecl { .. } => self.resolve_struct_decl(stmt),
            ParserDeclStmtKind::FuncDecl { .. } => self.resolve_func_decl(stmt),
            ParserDeclStmtKind::InfixDefine { .. } => self.resolve_infix_define(stmt),
            ParserDeclStmtKind::OperatorFunc { .. } => self.resolve_operator_func(stmt, op_type),
            ParserDeclStmtKind::StructField { name, .. } => {
                self.ec.top_level_struct_field(stmt.range, name)
            }
        }
    }
}
