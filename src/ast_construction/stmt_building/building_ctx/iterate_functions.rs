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

use crate::{ParserTopLevelStmtKind, Range, error::Ph, stmt_building::StmtBuildingCtx};

impl<'a> StmtBuildingCtx<'a> {
    /// Iterates over each function in the program and builds its body.
    pub fn build_func_bodies(&mut self) {
        let func_ids = self.program.funcs.keys().cloned().collect::<Vec<_>>();

        // Iterate over each function in the program
        for func_id in func_ids {
            // Get the TableStmtID used in the SymbolTable from the SymbolID used in the Program
            if let Some(func_path) = self.program.get_path_by_id(&func_id).cloned()
                && let Some(table_stmt_id) = self
                    .symbol_table
                    .get_id_by_path(&func_path)
                    .and_then(|ids| ids.first())
            {
                // Get the function body statements from the SymbolTable
                if let Some(func_body_stmts) =
                    match self.symbol_table.get_statement_by_id(table_stmt_id) {
                        Some(stmt) => match stmt.kind {
                            ParserTopLevelStmtKind::FuncDecl { ref body, .. } => Some(body),
                            _ => None,
                        },
                        None => None,
                    }
                {
                    // Add a function node to the graph
                    self.function_graph.add_node(func_id);

                    // Build the function body
                    let built_body = self.build_func_body_stmt(func_id, func_body_stmts);

                    match self.program.get_func_mut(&func_id) {
                        // Update the function body in the Program
                        Some(func) => func.body = built_body,
                        None => self.ec.comp_bug(
                            Range::zero(),
                            Ph::StatementBuilding,
                            &format!(
                                "Could not get function body from the Program for function ID {} and path {:?}",
                                func_id,
                                func_path
                            ),
                        ),
                    }
                } else {
                    self.ec.comp_bug(
                        Range::zero(),
                        Ph::StatementBuilding,
                        &format!(
                            "Could not get function body from the SymbolTable for function ID {} and path {:?}",
                            func_id,
                            func_path
                        ),
                    );
                }
            } else {
                self.ec.comp_bug(
                    Range::zero(),
                    Ph::StatementBuilding,
                    &format!(
                        "Function ID {} could not be resolved which should have been added",
                        func_id
                    ),
                );
            }
        }
    }
}
