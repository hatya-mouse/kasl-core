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
    ConstructorError, ConstructorErrorType, ParserStatementKind, Program, Range, SymbolTable,
    resolution::{
        dependency_analysis::{build_graph, sort_graph},
        expr_inference::ExprTypeInference,
        program_locator::ProgramLocator,
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

    // Resolve types for each symbol in the sorted order
    let mut errors = Vec::new();

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
                    if let Some(variable) = program.get_inferable_input_mut(&symbol_path) {
                        variable.value_type = type_symbol_path;
                    } else {
                        errors.push(ConstructorError {
                            error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                            position: symbol_decl_statement.range,
                        });
                    }
                } else if let Some(def_val) = def_val {
                    match program.infer_expr_type(def_val) {
                        Ok(type_symbol_path) => {
                            if let Some(variable) = program.get_inferable_input_mut(&symbol_path) {
                                variable.value_type = Some(type_symbol_path);
                            } else {
                                errors.push(ConstructorError {
                                    error_type: ConstructorErrorType::CannotInferType(
                                        symbol_path.clone(),
                                    ),
                                    position: symbol_decl_statement.range,
                                });
                            }
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
                if let Some(variable) = program.get_inferable_input_mut(&symbol_path) {
                    variable.value_type = type_symbol_path;
                } else {
                    errors.push(ConstructorError {
                        error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                        position: symbol_decl_statement.range,
                    });
                }
            }

            ParserStatementKind::State { vars } => {
                for var in vars {
                    if let Some(type_parser_path) = &var.value_type {
                        let type_symbol_path =
                            program.resolve_type_def_parser_path(&type_parser_path);
                        if let Some(variable) = program.get_inferable_var_mut(&symbol_path) {
                            variable.value_type = type_symbol_path;
                        } else {
                            errors.push(ConstructorError {
                                error_type: ConstructorErrorType::CannotInferType(
                                    symbol_path.clone(),
                                ),
                                position: symbol_decl_statement.range,
                            });
                        }
                    } else {
                        match program.infer_expr_type(&var.def_val) {
                            Ok(type_symbol_path) => {
                                if let Some(variable) = program.get_inferable_var_mut(&symbol_path)
                                {
                                    variable.value_type = Some(type_symbol_path);
                                } else {
                                    errors.push(ConstructorError {
                                        error_type: ConstructorErrorType::CannotInferType(
                                            symbol_path.clone(),
                                        ),
                                        position: symbol_decl_statement.range,
                                    });
                                }
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
                    if let Some(variable) = program.get_inferable_var_mut(&symbol_path) {
                        variable.value_type = type_symbol_path;
                    } else {
                        errors.push(ConstructorError {
                            error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                            position: symbol_decl_statement.range,
                        });
                    }
                } else if let Some(def_val) = def_val {
                    match program.infer_expr_type(def_val) {
                        Ok(type_symbol_path) => {
                            if let Some(variable) = program.get_inferable_var_mut(&symbol_path) {
                                variable.value_type = Some(type_symbol_path);
                            } else {
                                errors.push(ConstructorError {
                                    error_type: ConstructorErrorType::CannotInferType(
                                        symbol_path.clone(),
                                    ),
                                    position: symbol_decl_statement.range,
                                });
                            }
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
                        if let Some(variable) = program.get_inferable_var_mut(&symbol_path) {
                            variable.value_type = type_symbol_path;
                        } else {
                            errors.push(ConstructorError {
                                error_type: ConstructorErrorType::CannotInferType(
                                    symbol_path.clone(),
                                ),
                                position: symbol_decl_statement.range,
                            });
                        }
                    } else if let Some(def_val) = &param.def_val {
                        match program.infer_expr_type(def_val) {
                            Ok(type_symbol_path) => {
                                if let Some(variable) = program.get_inferable_var_mut(&symbol_path)
                                {
                                    variable.value_type = Some(type_symbol_path);
                                } else {
                                    errors.push(ConstructorError {
                                        error_type: ConstructorErrorType::CannotInferType(
                                            symbol_path.clone(),
                                        ),
                                        position: symbol_decl_statement.range,
                                    });
                                }
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
