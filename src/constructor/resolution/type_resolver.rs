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
    ConstructorError, ConstructorErrorType, ExprToken, ParserOperatorType, ParserStatementKind,
    ParserSymbolPathComponent, Program, Range, SymbolPath, SymbolTable,
    ast::tree_items::variables::VariableTrait,
    resolution::{
        dependency_analysis::{build_graph, sort_graph},
        expr_inference::ExprTreeBuilder,
        resolvers::{
            infix_operator_resolver::resolve_infix_func,
            prefix_operator_resolver::resolve_prefix_operator,
        },
    },
    symbol_table::symbol_table::StatementLookup,
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
            let mut errors = Vec::new();
            for symbol_path in causative_symbols {
                if let Some(symbol_decl_statement) =
                    symbol_table.get_statement_by_path(&symbol_path)
                {
                    match symbol_decl_statement {
                        StatementLookup::Single(stmt) => {
                            // And get the range in which the statement is declared
                            errors.push(ConstructorError {
                                error_type: ConstructorErrorType::DependencyCycle(symbol_path),
                                position: stmt.range,
                            });
                        }
                        StatementLookup::Multiple(stmts) => {
                            // Iterate over each statement and push an error for each one
                            for stmt in stmts {
                                errors.push(ConstructorError {
                                    error_type: ConstructorErrorType::DependencyCycle(
                                        symbol_path.clone(),
                                    ),
                                    position: stmt.range,
                                });
                            }
                        }
                    }
                } else {
                    errors.push(ConstructorError {
                        error_type: ConstructorErrorType::DependencyCycle(symbol_path),
                        position: Range::zero(),
                    });
                }
            }
            return Err(errors);
        }
    };

    let mut errors = Vec::new();

    let mut statements = Vec::new();
    for symbol_path in sorted_list {
        // Get the symbol declaration statement
        if let Some(symbol_decl_statement) = symbol_table.get_statement_by_path(&symbol_path) {
            match symbol_decl_statement {
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
            errors.push(ConstructorError {
                error_type: ConstructorErrorType::SymbolNotFound(None),
                position: Range::zero(),
            });
        }
    }

    // Infer the type of each symbol in the sorted order
    for (symbol_path, symbol_decl_statement) in statements {
        // Check if the symbol has already got a type annotation
        // If not, infer the type
        match &symbol_decl_statement.kind {
            ParserStatementKind::Input {
                name,
                value_type,
                def_val,
                attrs: _,
            } => match infer_type_and_write(
                program,
                symbol_table,
                symbol_path,
                symbol_decl_statement.range,
                |program| program.get_input_mut(name),
                Some(def_val),
                value_type.as_ref(),
            ) {
                Ok(()) => (),
                Err(errs) => errors.extend(errs),
            },

            ParserStatementKind::Output {
                name,
                value_type,
                def_val,
            } => match infer_type_and_write(
                program,
                symbol_table,
                symbol_path,
                symbol_decl_statement.range,
                |program| program.get_output_mut(name),
                Some(def_val),
                value_type.as_ref(),
            ) {
                Ok(()) => (),
                Err(errs) => errors.extend(errs),
            },

            ParserStatementKind::State { vars } => {
                for var in vars {
                    match infer_type_and_write(
                        program,
                        symbol_table,
                        symbol_path,
                        symbol_decl_statement.range,
                        |program| program.get_state_mut(&var.name),
                        Some(&var.def_val),
                        var.value_type.as_ref(),
                    ) {
                        Ok(()) => (),
                        Err(errs) => errors.extend(errs),
                    }
                }
            }

            ParserStatementKind::Var {
                required_by: _,
                name: _,
                value_type,
                def_val,
            } => match infer_type_and_write(
                program,
                symbol_table,
                symbol_path,
                symbol_decl_statement.range,
                |program| program.get_var_by_path_mut(symbol_path),
                Some(def_val),
                value_type.as_ref(),
            ) {
                Ok(()) => (),
                Err(errs) => errors.extend(errs),
            },

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
                    match infer_type_and_write(
                        program,
                        symbol_table,
                        symbol_path,
                        symbol_decl_statement.range,
                        |program| program.get_func_param_by_path_mut(symbol_path, &param.name),
                        param.def_val.as_ref(),
                        param.value_type.as_ref(),
                    ) {
                        Ok(_) => (),
                        Err(errs) => errors.extend(errs),
                    }
                }
            }

            ParserStatementKind::OperatorFunc {
                op_type,
                symbol,
                params,
                return_type,
                body: _,
            } => match op_type {
                ParserOperatorType::Infix => match resolve_infix_func(
                    program,
                    symbol,
                    symbol_path,
                    params,
                    return_type,
                    symbol_decl_statement.range,
                ) {
                    Ok(_) => (),
                    Err(err) => errors.push(err),
                },
                ParserOperatorType::Prefix => match resolve_prefix_operator(
                    program,
                    symbol,
                    symbol_path,
                    params,
                    return_type,
                    symbol_decl_statement.range,
                ) {
                    Ok(_) => (),
                    Err(err) => errors.push(err),
                },
            },

            _ => (),
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Infer the type of the variable and write it to the Program.
///
/// # Arguments
/// - `program`: Mutable reference to the Program.
/// - `symbol_path`: SymbolPath of the variable to infer type for.
/// - `decl_range`: Range of the declaration statement.
/// - `get_target`: Function to get the target variable.
fn infer_type_and_write<T, F>(
    program: &mut Program,
    symbol_table: &SymbolTable,
    symbol_path: &SymbolPath,
    decl_range: Range,
    get_target: F,
    default_value: Option<&Vec<ExprToken>>,
    type_annotation: Option<&Vec<ParserSymbolPathComponent>>,
) -> Result<(), Vec<ConstructorError>>
where
    F: for<'a> Fn(&'a mut Program) -> Option<&'a mut T>,
    T: VariableTrait + Sized + std::fmt::Debug,
{
    if let Some(default_value) = default_value {
        let parsed_expr = program.build_expr_tree_from_raw_tokens(default_value, symbol_table)?;
        let parsed_expr_type = match parsed_expr.get_type(program) {
            Some(t) => t,
            None => {
                return Err(vec![ConstructorError {
                    error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                    position: decl_range,
                }]);
            }
        };

        {
            let target_variable = get_target(program).ok_or_else(|| {
                vec![ConstructorError {
                    error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                    position: decl_range,
                }]
            })?;
            target_variable.set_default_value(Some(parsed_expr));
        }

        if let Some(type_annotation) = type_annotation {
            if let Some(type_symbol_path) = program.resolve_type_def_parser_path(type_annotation) {
                // If the symbol has a type annotation, use it
                if type_symbol_path == parsed_expr_type {
                    let target_variable = get_target(program).ok_or_else(|| {
                        vec![ConstructorError {
                            error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                            position: decl_range,
                        }]
                    })?;
                    target_variable.set_value_type(Some(type_symbol_path));
                } else {
                    // If the type annotation doesn't match the inferred type, push an error
                    return Err(vec![ConstructorError {
                        error_type: ConstructorErrorType::TypeMismatch(
                            parsed_expr_type,
                            type_symbol_path,
                        ),
                        position: decl_range,
                    }]);
                }
            } else {
                // If the type annotation is not found, push an error
                return Err(vec![ConstructorError {
                    error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                    position: decl_range,
                }]);
            }
        } else {
            // If the symbol doesn't have a type annotation, use the inferred one
            let target_variable = get_target(program).ok_or_else(|| {
                vec![ConstructorError {
                    error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                    position: decl_range,
                }]
            })?;
            target_variable.set_value_type(Some(parsed_expr_type));
        }
    } else {
        if let Some(type_annotation) = type_annotation {
            if let Some(type_symbol_path) = program.resolve_type_def_parser_path(type_annotation) {
                // If the symbol has a type annotation, use it
                let target_variable = get_target(program).ok_or_else(|| {
                    vec![ConstructorError {
                        error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                        position: decl_range,
                    }]
                })?;
                target_variable.set_value_type(Some(type_symbol_path));
            } else {
                // If the type annotation is not found, push an error
                return Err(vec![ConstructorError {
                    error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                    position: decl_range,
                }]);
            }
        } else {
            // If the symbol doesn't have the both type annotation and inferred type, push an error
            return Err(vec![ConstructorError {
                error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                position: decl_range,
            }]);
        }
    }

    Ok(())
}
