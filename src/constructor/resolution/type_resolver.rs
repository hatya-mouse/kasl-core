//
// Copyright 2025 Shuntaro Kasatani
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
    ConstructorError, ConstructorErrorType, ParserStatementKind, Program, Range, SymbolPath,
    SymbolTable,
    ast::tree_items::variables::VariableTrait,
    resolution::{
        dependency_analysis::{build_graph, sort_graph},
        expr_inference::ExprTypeInference,
    },
};

pub fn resolve_types(
    program: &mut Program,
    symbol_table: &SymbolTable,
) -> Result<(), Vec<ConstructorError>> {
    // Build the type dependency graph
    let graph = build_graph(symbol_table);

    // Then sort symbols based on the dependency graph
    let sorted_list = match sort_graph(&graph) {
        Ok(sorted_list) => sorted_list,
        Err(causative_symbols) => {
            let errors = causative_symbols
                .into_iter()
                .map(|symbol| {
                    // Get the symbol declaration statement
                    let symbol_decl_statement = symbol_table.get_statement_by_path(&symbol);
                    // And get the range in which the statement is declared
                    let symbol_decl_position = symbol_decl_statement.map(|stmt| stmt.range);
                    ConstructorError {
                        error_type: ConstructorErrorType::DependencyCycle(symbol),
                        position: symbol_decl_position.unwrap_or(Range::zero()),
                    }
                })
                .collect();
            return Err(errors);
        }
    };

    let mut errors = Vec::new();

    // Infer the type of each symbol in the sorted order
    for symbol_path in sorted_list {
        // Get the symbol declaration statement
        let symbol_decl_statement = match symbol_table.get_statement_by_path(&symbol_path) {
            Some(stmt) => stmt,
            None => {
                continue;
            }
        };

        // Check if the symbol has already got a type annotation
        // If not, infer the type
        match &symbol_decl_statement.kind {
            ParserStatementKind::Input {
                name: _,
                value_type,
                def_val,
                attrs: _,
            } => {
                if let Some(type_parser_path) = value_type {
                    let type_symbol_path = program.resolve_type_def_parser_path(&type_parser_path);
                    process_inference_write(
                        program,
                        &mut errors,
                        symbol_path,
                        symbol_decl_statement.range,
                        Program::get_input_by_path_mut,
                        type_symbol_path,
                    );
                } else if let Some(def_val) = def_val {
                    match program.infer_expr_type(def_val, symbol_table) {
                        Ok(type_symbol_path) => {
                            process_inference_write(
                                program,
                                &mut errors,
                                symbol_path,
                                symbol_decl_statement.range,
                                Program::get_input_by_path_mut,
                                Some(type_symbol_path),
                            );
                        }
                        Err(err) => errors.push(err),
                    }
                }
            }

            ParserStatementKind::Output {
                name: _,
                value_type,
            } => {
                let type_symbol_path = program.resolve_type_def_parser_path(&value_type);
                process_inference_write(
                    program,
                    &mut errors,
                    symbol_path,
                    symbol_decl_statement.range,
                    Program::get_output_by_path_mut,
                    type_symbol_path,
                );
            }

            ParserStatementKind::State { vars } => {
                for var in vars {
                    if let Some(type_parser_path) = &var.value_type {
                        let type_symbol_path =
                            program.resolve_type_def_parser_path(&type_parser_path);
                        process_inference_write(
                            program,
                            &mut errors,
                            symbol_path,
                            symbol_decl_statement.range,
                            Program::get_state_by_path_mut,
                            type_symbol_path,
                        );
                    } else {
                        match program.infer_expr_type(&var.def_val, symbol_table) {
                            Ok(type_symbol_path) => {
                                process_inference_write(
                                    program,
                                    &mut errors,
                                    symbol_path,
                                    symbol_decl_statement.range,
                                    Program::get_state_by_path_mut,
                                    Some(type_symbol_path),
                                );
                            }
                            Err(err) => errors.push(err),
                        }
                    }
                }
            }

            ParserStatementKind::Var {
                required_by: _,
                name: _,
                value_type,
                def_val,
            } => {
                if let Some(type_parser_path) = value_type {
                    let type_symbol_path = program.resolve_type_def_parser_path(&type_parser_path);
                    process_inference_write(
                        program,
                        &mut errors,
                        symbol_path,
                        symbol_decl_statement.range,
                        Program::get_var_by_path_mut,
                        type_symbol_path,
                    );
                } else if let Some(def_val) = def_val {
                    match program.infer_expr_type(def_val, symbol_table) {
                        Ok(type_symbol_path) => {
                            process_inference_write(
                                program,
                                &mut errors,
                                symbol_path,
                                symbol_decl_statement.range,
                                Program::get_var_by_path_mut,
                                Some(type_symbol_path),
                            );
                        }
                        Err(err) => errors.push(err),
                    }
                }
            }

            ParserStatementKind::FuncDecl {
                required_by: _,
                name: _,
                params,
                return_type: _,
                body: _,
            } => {
                for param in params {
                    if let Some(type_parser_path) = &param.value_type {
                        let type_symbol_path =
                            program.resolve_type_def_parser_path(&type_parser_path);
                        process_inference_write(
                            program,
                            &mut errors,
                            symbol_path,
                            symbol_decl_statement.range,
                            Program::get_func_param_by_path_mut,
                            type_symbol_path,
                        );
                    } else if let Some(def_val) = &param.def_val {
                        match program.infer_expr_type(def_val, symbol_table) {
                            Ok(type_symbol_path) => {
                                process_inference_write(
                                    program,
                                    &mut errors,
                                    symbol_path,
                                    symbol_decl_statement.range,
                                    Program::get_var_by_path_mut,
                                    Some(type_symbol_path),
                                );
                            }
                            Err(err) => errors.push(err),
                        }
                    }
                }
            }

            _ => (),
        }
    }

    Ok(())
}

/// Process operation for writing inferred type to the Program.
///
/// # Arguments
/// - `program`: Mutable reference to the Program.
/// - `symbol_path`: SymbolPath of the variable to infer type for.
/// - `decl_range`: Range of the declaration statement.
/// - `get_target`: Function to get the target variable.
/// - `inferred_type`: SymbolPath of the inferred type.
fn process_inference_write<T, F>(
    program: &mut Program,
    errors: &mut Vec<ConstructorError>,
    symbol_path: &SymbolPath,
    decl_range: Range,
    get_target: F,
    inferred_type: Option<SymbolPath>,
) where
    F: for<'a> FnOnce(&'a mut Program, &SymbolPath) -> Option<&'a mut T>,
    T: VariableTrait + Sized,
{
    if inferred_type.is_some() {
        match get_target(program, &symbol_path) {
            Some(target) => {
                target.set_value_type(inferred_type);
            }
            None => errors.push(ConstructorError {
                error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                position: decl_range,
            }),
        }
    } else {
        errors.push(ConstructorError {
            error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
            position: decl_range,
        });
    }
}
