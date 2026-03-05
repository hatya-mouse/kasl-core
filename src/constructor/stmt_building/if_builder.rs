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
    IfArm, ParserBodyStmt, ParserIfArm, Program, Statement, SymbolTable, error::ErrorCollector,
    resolution::expr_inference::ExprTreeBuilder,
    stmt_building::func_body_builder::build_func_body_stmt,
};

pub fn build_if_stmt(
    ec: &mut ErrorCollector,
    program: &mut Program,
    symbol_table: &SymbolTable,
    parsed_stmts: &mut Vec<Statement>,
    parser_main: &ParserIfArm,
    parser_else_ifs: &Vec<ParserIfArm>,
    parser_else_body: &[ParserBodyStmt],
) {
    // Parse the main (if) arm
    let main = match build_if_arm(ec, program, symbol_table, parser_main) {
        Some(arm) => arm,
        None => return,
    };

    // Parse each arms in the parser_else_ifs
    let mut else_ifs = Vec::new();
    for parser_arm in parser_else_ifs {
        match build_if_arm(ec, program, symbol_table, parser_arm) {
            Some(arm) => else_ifs.push(arm),
            None => continue,
        }
    }

    // Parse the else body
    let else_body = build_func_body_stmt(ec, program, symbol_table, parser_else_body);

    let if_stmt = Statement::If {
        main,
        else_ifs,
        else_body,
    };

    // Push the statement onto the parsed_stmts
    parsed_stmts.push(if_stmt);
}

pub fn build_if_arm(
    ec: &mut ErrorCollector,
    program: &mut Program,
    symbol_table: &SymbolTable,
    parser_arm: &ParserIfArm,
) -> Option<IfArm> {
    // Parse the main (if) condition
    let condition =
        match program.build_expr_tree_from_raw_tokens(ec, &parser_arm.condition, symbol_table) {
            Some(parsed_value) => parsed_value,
            None => {
                // Error should have been reported in the build_expr_tree_from_raw_tokens function so we don't need to report it here
                return None;
            }
        };

    // Collect the main body statements
    let body = build_func_body_stmt(ec, program, symbol_table, &parser_arm.body);

    Some(IfArm::new(condition, body))
}
