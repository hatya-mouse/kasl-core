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
    ParserBodyStmt, ParserBodyStmtKind, Program, Statement, SymbolTable,
    error::{ErrorCollector, Phase},
    resolution::expr_inference::ExprTreeBuilder,
    stmt_building::func_call_builder::build_func_call_stmt,
};

pub fn build_func_body_stmt(
    ec: &mut ErrorCollector,
    program: &Program,
    symbol_table: &SymbolTable,
    original_stmts: &[ParserBodyStmt],
) -> Vec<Statement> {
    let mut parsed_stmts = Vec::new();

    for stmt in original_stmts {
        match &stmt.kind {
            ParserBodyStmtKind::Assign { target, value } => {
                let parsed_target = match symbol_table.resolve_path(target) {
                    Some(parsed_target) => parsed_target,
                    None => {
                        ec.var_not_found(stmt.range, Phase::StatementBuilding, &target.to_string());
                        continue;
                    }
                };

                let parsed_value =
                    match program.build_expr_tree_from_raw_tokens(ec, value, symbol_table) {
                        Some(parsed_value) => parsed_value,
                        None => {
                            // Error should have been reported the function so we don't need to report it here
                            continue;
                        }
                    };

                // Create an Assign statement
                let assign_stmt = Statement::Assign {
                    target: parsed_target,
                    value: parsed_value,
                };
                parsed_stmts.push(assign_stmt);
            }

            ParserBodyStmtKind::Block { statements } => {
                // Build statements within the block
                let block_body = build_func_body_stmt(ec, program, symbol_table, statements);
                // Create a Block statement
                let block_stmt = Statement::Block { body: block_body };
                parsed_stmts.push(block_stmt);
            }

            ParserBodyStmtKind::FuncCall { path, args } => build_func_call_stmt(
                ec,
                program,
                symbol_table,
                &mut parsed_stmts,
                stmt,
                path,
                args,
            ),

            _ => (),
        }
    }

    parsed_stmts
}
