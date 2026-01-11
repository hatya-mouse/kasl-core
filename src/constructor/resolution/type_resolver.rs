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
    ConstructorError, ConstructorErrorType, ParserStatementKind, Program, Range, SymbolPath,
    SymbolTable,
    ast::tree_items::variables::VariableTrait,
    resolution::{
        dependency_analysis::{build_graph, sort_graph},
        expr_inference::expr_type_inference::ExprTypeInference,
    },
};

/// Infer the types of symbols (input, output, state, var, and function parameters) in the program.
pub fn resolve_types(
    program: &mut Program,
    symbol_table: &SymbolTable,
) -> Result<(), Vec<ConstructorError>> {
    // Build the type dependency graph
    let graph = build_graph(symbol_table).map_err(|err| vec![err])?;

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
                name,
                value_type,
                def_val,
                attrs: _,
            } => {
                if let Some(type_symbol_path) = resolve_type_or_push_error(
                    program,
                    &mut errors,
                    value_type.as_ref(),
                    symbol_decl_statement.range,
                ) {
                    // If the symbol has a type annotation, use it
                    match program.get_input_mut(name) {
                        Some(input) => input.value_type = Some(type_symbol_path),
                        None => errors.push(ConstructorError {
                            error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                            position: symbol_decl_statement.range,
                        }),
                    }
                } else if let Some(def_val) = def_val {
                    // If the symbol doesn't have a type annotation, infer it from the default value
                    match program.infer_expr_type(def_val, symbol_table) {
                        Ok(type_symbol_path) => match program.get_input_mut(name) {
                            Some(input) => input.value_type = Some(type_symbol_path),
                            None => errors.push(ConstructorError {
                                error_type: ConstructorErrorType::CannotInferType(
                                    symbol_path.clone(),
                                ),
                                position: symbol_decl_statement.range,
                            }),
                        },
                        Err(err) => errors.push(err),
                    }
                }
            }

            ParserStatementKind::Output { name, value_type } => {
                // Output variable must have a type annotation
                if let Some(type_symbol_path) = program.resolve_type_def_parser_path(&value_type) {
                    match program.get_output_mut(name) {
                        Some(output) => output.value_type = Some(type_symbol_path),
                        None => errors.push(ConstructorError {
                            error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                            position: symbol_decl_statement.range,
                        }),
                    }
                } else {
                    errors.push(ConstructorError {
                        error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                        position: symbol_decl_statement.range,
                    });
                }
            }

            ParserStatementKind::State { vars } => {
                for var in vars {
                    if let Some(type_symbol_path) = resolve_type_or_push_error(
                        program,
                        &mut errors,
                        var.value_type.as_ref(),
                        symbol_decl_statement.range,
                    ) {
                        // If the symbol has a type annotation, use it
                        match program.get_state_mut(&var.name) {
                            Some(state) => state.value_type = Some(type_symbol_path),
                            None => errors.push(ConstructorError {
                                error_type: ConstructorErrorType::CannotInferType(
                                    symbol_path.clone(),
                                ),
                                position: symbol_decl_statement.range,
                            }),
                        }
                    } else {
                        // If the symbol doesn't have a type annotation, infer it from the default value
                        match program.infer_expr_type(&var.def_val, symbol_table) {
                            Ok(type_symbol_path) => match program.get_state_mut(&var.name) {
                                Some(state) => state.value_type = Some(type_symbol_path),
                                None => errors.push(ConstructorError {
                                    error_type: ConstructorErrorType::CannotInferType(
                                        symbol_path.clone(),
                                    ),
                                    position: symbol_decl_statement.range,
                                }),
                            },
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
                    // If the symbol has a type annotation, use it
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
                    // If the symbol doesn't have a type annotation, infer it from the default value
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
                name,
                params,
                return_type,
                body: _,
            } => {
                // If the function has a return type, resolve it
                if let Some(return_type) = return_type {
                    if let Some(return_type_path) =
                        program.resolve_type_def_parser_path(&return_type)
                    {
                        match program.get_func_mut(name) {
                            Some(func) => func.return_type = Some(return_type_path),
                            None => errors.push(ConstructorError {
                                error_type: ConstructorErrorType::CannotInferType(
                                    symbol_path.clone(),
                                ),
                                position: symbol_decl_statement.range,
                            }),
                        }
                    } else {
                        errors.push(ConstructorError {
                            error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                            position: symbol_decl_statement.range,
                        });
                    }
                }

                for param in params {
                    if let Some(type_symbol_path) = resolve_type_or_push_error(
                        program,
                        &mut errors,
                        param.value_type.as_ref(),
                        symbol_decl_statement.range,
                    ) {
                        // If the symbol has a type annotation, use it
                        process_inference_write(
                            program,
                            &mut errors,
                            symbol_path,
                            symbol_decl_statement.range,
                            Program::get_func_param_by_path_mut,
                            Some(type_symbol_path),
                        );
                    } else if let Some(def_val) = &param.def_val {
                        // If the symbol doesn't have a type annotation, infer it from the default value
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

/// Try resolve a parser type path to a SymbolPath; on failure push an appropriate error into `errors` and return None.
///
/// - `program`: Program reference
/// - `errors`: error vec to push into
/// - `type_parser_path_opt`: Option<&ParserSymbolPath> — if None, returns None (caller handles inference-from-default-value)
/// - `symbol_path`: the declaration symbol path (for error construction)
/// - `decl_range`: source range for error position
fn resolve_type_or_push_error(
    program: &Program,
    errors: &mut Vec<ConstructorError>,
    type_parser_path_opt: Option<&crate::ParserSymbolPath>,
    decl_range: Range,
) -> Option<SymbolPath> {
    match type_parser_path_opt {
        Some(type_parser_path) => match program.resolve_type_def_parser_path(type_parser_path) {
            Some(tp) => Some(tp),
            None => {
                // Type name not found: treat as user-level symbol-not-found error
                errors.push(ConstructorError {
                    error_type: ConstructorErrorType::SymbolNotFound(None),
                    position: decl_range,
                });
                None
            }
        },
        None => None,
    }
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
