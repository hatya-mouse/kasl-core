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
    ParserOperatorType, ParserTopLevelStmt, ParserTopLevelStmtKind, Program, Range, SymbolPath,
    SymbolTable,
    error::{ErrorCollector, Phase},
    resolution::{
        TypeResolveCtx,
        dependency_analysis::{build_graph, sort_graph},
    },
    table_construction::symbol_table::StatementLookup,
};

/// Infer the types of symbols (input, output, state, var, and function parameters) in the program.
pub fn resolve_types(ec: &mut ErrorCollector, program: &mut Program, symbol_table: &SymbolTable) {
    // Build the type dependency graph
    let graph = match build_graph(ec, symbol_table) {
        Some(graph) => graph,
        None => return,
    };

    // Then sort symbols based on the dependency graph
    let sorted_list = match sort_graph(ec, symbol_table, &graph) {
        Some(sorted_list) => sorted_list,
        None => return,
    };

    // Get references from the sorted symbol paths
    let statements = get_statements(ec, symbol_table, sorted_list);

    // Create a TypeResolveCtx instance
    let mut ctx = TypeResolveCtx::new(ec, program, symbol_table);

    // Infer the type of each symbol in the sorted order
    for (symbol_path, current_stmt) in statements {
        // Check if the symbol has already got a type annotation
        // If not, infer the type
        match &current_stmt.kind {
            ParserTopLevelStmtKind::Input {
                name,
                value_type,
                def_val,
                attrs,
            } => ctx.resolve_input(
                name,
                value_type.as_ref(),
                def_val,
                attrs,
                current_stmt.range,
            ),

            ParserTopLevelStmtKind::Output {
                name,
                value_type,
                def_val,
            } => ctx.resolve_output(name, value_type.as_ref(), def_val, current_stmt.range),

            ParserTopLevelStmtKind::State { vars } => {
                for var in vars {
                    ctx.resolve_state(
                        &var.name,
                        var.value_type.as_ref(),
                        &var.def_val,
                        current_stmt.range,
                    );
                }
            }

            ParserTopLevelStmtKind::GlobalVar {
                name,
                value_type,
                def_val,
            } => ctx.resolve_var(
                name,
                symbol_path,
                value_type.as_ref(),
                def_val,
                current_stmt.range,
            ),

            ParserTopLevelStmtKind::FuncDecl {
                name,
                params,
                return_type,
                body: _,
            } => ctx.resolve_func(
                name,
                symbol_path,
                params,
                return_type.as_ref(),
                current_stmt.range,
            ),

            ParserTopLevelStmtKind::Init {
                literal_bind,
                params,
                body: _,
            } => ctx.resolve_init(
                symbol_path,
                literal_bind.as_ref(),
                params,
                current_stmt.range,
            ),

            ParserTopLevelStmtKind::OperatorFunc {
                op_type,
                symbol,
                params,
                return_type,
                body: _,
            } => match op_type {
                ParserOperatorType::Infix => {
                    ctx.resolve_infix_func(symbol, params, return_type, current_stmt.range)
                }
                ParserOperatorType::Prefix => {
                    ctx.resolve_prefix_operator(symbol, params, return_type, current_stmt.range)
                }
            },

            ParserTopLevelStmtKind::InfixDefine {
                symbol,
                infix_properties,
            } => ctx.register_infix_define(symbol, infix_properties.clone()),

            ParserTopLevelStmtKind::PrefixDefine { symbol } => ctx.register_prefix_define(symbol),

            _ => (),
        }
    }
}

fn get_statements<'a>(
    ec: &mut ErrorCollector,
    symbol_table: &'a SymbolTable,
    symbol_paths: Vec<&'a SymbolPath>,
) -> Vec<(&'a SymbolPath, &'a ParserTopLevelStmt)> {
    let mut statements = Vec::new();

    for symbol_path in symbol_paths {
        // Get the symbol declaration statement
        if let Some(current_stmt) = symbol_table.get_statement_by_path(symbol_path) {
            match current_stmt {
                StatementLookup::Single(stmt) => {
                    statements.push((symbol_path, stmt));
                }
                StatementLookup::Multiple(stmts) => {
                    for stmt in stmts {
                        statements.push((symbol_path, stmt));
                    }
                }
            }
        } else {
            ec.comp_bug(
                Range::zero(),
                Phase::GraphConstruction,
                &format!(
                    "SymbolPath(s) in the dependency graph must be valid: {:?}",
                    symbol_path
                ),
            );
        }
    }

    statements
}
