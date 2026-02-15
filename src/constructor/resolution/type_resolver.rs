//
// Copyright 2025-2026 Shuntaro Kasatani
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
    ParserOperatorType, ParserStatementKind, Program, Range, SymbolTable,
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
    let sorted_list = match sort_graph(&graph) {
        Ok(sorted_list) => sorted_list,
        Err(causative_symbols) => {
            for symbol_path in causative_symbols {
                if let Some(current_stmt) = symbol_table.get_statement_by_path(&symbol_path) {
                    match current_stmt {
                        StatementLookup::Single(stmt) => {
                            // And get the range in which the statement is declared
                            ec.dep_cycle(
                                stmt.range,
                                Phase::TypeResolution,
                                &symbol_path.to_string(),
                            );
                        }
                        StatementLookup::Multiple(stmts) => {
                            // Iterate over each statement and push an error for each one
                            for stmt in stmts {
                                ec.dep_cycle(
                                    stmt.range,
                                    Phase::TypeResolution,
                                    &symbol_path.to_string(),
                                );
                            }
                        }
                    }
                } else {
                    ec.dep_cycle(
                        Range::zero(),
                        Phase::TypeResolution,
                        &symbol_path.to_string(),
                    );
                }
            }
            return;
        }
    };

    let mut statements = Vec::new();
    for symbol_path in sorted_list {
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
            ec.comp_bug(Range::zero(), Phase::TypeResolution, "");
        }
    }

    let mut ctx = TypeResolveCtx::new(ec, program, symbol_table);

    // Infer the type of each symbol in the sorted order
    for (symbol_path, current_stmt) in statements {
        // Check if the symbol has already got a type annotation
        // If not, infer the type
        match &current_stmt.kind {
            ParserStatementKind::Input {
                name,
                value_type,
                def_val,
                attrs: _,
            } => ctx.resolve_variable(
                symbol_path,
                current_stmt.range,
                |program| program.get_input_mut(name),
                Some(def_val),
                value_type.as_ref(),
            ),

            ParserStatementKind::Output {
                name,
                value_type,
                def_val,
            } => ctx.resolve_variable(
                symbol_path,
                current_stmt.range,
                |program| program.get_output_mut(name),
                Some(def_val),
                value_type.as_ref(),
            ),

            ParserStatementKind::State { vars } => {
                for var in vars {
                    ctx.resolve_variable(
                        symbol_path,
                        current_stmt.range,
                        |program| program.get_state_mut(&var.name),
                        Some(&var.def_val),
                        var.value_type.as_ref(),
                    );
                }
            }

            ParserStatementKind::Var {
                required_by: _,
                name: _,
                value_type,
                def_val,
            } => ctx.resolve_variable(
                symbol_path,
                current_stmt.range,
                |program| program.get_var_by_path_mut(symbol_path),
                Some(def_val),
                value_type.as_ref(),
            ),

            ParserStatementKind::FuncDecl {
                required_by,
                name,
                params,
                return_type,
                body: _,
            } => ctx.resolve_func(
                name,
                symbol_path,
                params,
                required_by.as_ref(),
                return_type.as_ref(),
                current_stmt.range,
            ),

            ParserStatementKind::OperatorFunc {
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

            _ => (),
        }
    }
}
