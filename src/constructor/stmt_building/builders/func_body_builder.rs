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
    resolution::{TypeResolveCtx, expr_inference::ExprTreeBuilder},
    stmt_building::builders::{func_call_builder::build_func_call_stmt, if_builder::build_if_stmt},
};

pub fn build_func_body_stmt(
    ec: &mut ErrorCollector,
    program: &mut Program,
    symbol_table: &SymbolTable,
    original_stmts: &[ParserBodyStmt],
) -> Vec<Statement> {
    let mut parsed_stmts = Vec::new();

    for stmt in original_stmts {
        match &stmt.kind {
            ParserBodyStmtKind::Assign { target, value } => {
                let parsed_target = match program
                    .get_id_by_path(target)
                    .and_then(|ids| ids.first().cloned())
                {
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
                            // Error should have been reported in the build_expr_tree_from_raw_tokens function so we don't need to report it here
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
                // Collect statements within the block
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

            ParserBodyStmtKind::LocalVar {
                name,
                value_type,
                def_val,
            } => {
                let mut ctx = TypeResolveCtx::new(ec, program, symbol_table);
                if let Some((val_type, def_val)) =
                    ctx.resolve_var_type(stmt.range, value_type.as_ref(), def_val)
                {
                    let var_decl = Statement::VarDecl {
                        name: name.clone(),
                        value_type: val_type,
                        def_val,
                    };
                    parsed_stmts.push(var_decl);
                }
            }

            ParserBodyStmtKind::If {
                main,
                else_ifs,
                else_body,
            } => build_if_stmt(
                ec,
                program,
                symbol_table,
                &mut parsed_stmts,
                main,
                else_ifs,
                else_body,
            ),

            ParserBodyStmtKind::Return { value } => {
                let return_value = value.as_ref().and_then(|value| {
                    program.build_expr_tree_from_raw_tokens(ec, value, symbol_table)
                });
                let return_stmt = Statement::Return {
                    value: return_value,
                };
                parsed_stmts.push(return_stmt);
            }
        }
    }

    parsed_stmts
}
