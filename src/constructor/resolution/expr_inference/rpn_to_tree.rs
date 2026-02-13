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
    ConstructorError, ConstructorErrorType, ExprTokenKind, Expression, FuncCallArg, Program, Range,
    SymbolTable, TypedToken, TypedTokenKind, resolution::expr_inference::ExprTreeBuilder,
};

/// Convert an RPN sequence of TypedToken into an Expression tree.
/// Returns Err on arity errors, unknown symbols, or missing operator implementations.
pub fn build_expr_tree_from_rpn(
    program: &Program,
    symbol_table: &SymbolTable,
    rpn_tokens: Vec<TypedToken>,
) -> Result<Expression, Vec<ConstructorError>> {
    let mut stack = Vec::new();

    for current_token in rpn_tokens.into_iter() {
        match current_token.kind {
            TypedTokenKind::Value {
                expr_token,
                value_type,
            } => match expr_token.kind {
                ExprTokenKind::IntLiteral(value) => {
                    stack.push((Expression::IntLiteral(value), value_type))
                }
                ExprTokenKind::FloatLiteral(value) => {
                    stack.push((Expression::FloatLiteral(value), value_type))
                }
                ExprTokenKind::BoolLiteral(value) => {
                    stack.push((Expression::BoolLiteral(value), value_type))
                }
                ExprTokenKind::Identifier(ref path) => {
                    let resolved_path = symbol_table.resolve_path(path).ok_or_else(|| {
                        vec![ConstructorError {
                            error_type: ConstructorErrorType::SymbolNotFound(None),
                            position: current_token.range,
                        }]
                    })?;
                    stack.push((Expression::Identifier(resolved_path), value_type))
                }
                ExprTokenKind::FuncCall { ref path, args } => {
                    let resolved_func_path = symbol_table.resolve_path(path).ok_or_else(|| {
                        vec![ConstructorError {
                            error_type: ConstructorErrorType::SymbolNotFound(None),
                            position: current_token.range,
                        }]
                    })?;

                    let function =
                        program
                            .get_func_by_path(&resolved_func_path)
                            .ok_or_else(|| {
                                vec![ConstructorError {
                                    error_type: ConstructorErrorType::SymbolNotFound(Some(
                                        resolved_func_path.clone(),
                                    )),
                                    position: current_token.range,
                                }]
                            })?;

                    // Build expression tree for arguments
                    let mut parsed_arguments = Vec::new();
                    let mut errors = Vec::new();
                    for i in 0..args.len() {
                        let arg = &args[i];

                        // Check if the function has enough number of arguments
                        if i < function.params.len() {
                            // Build expression tree for an argument
                            let tree_expr = match program
                                .build_expr_tree_from_raw_tokens(&arg.value, symbol_table)
                            {
                                Ok(expr) => expr,
                                Err(err) => {
                                    errors.extend(err);
                                    break;
                                }
                            };

                            // Check if the argument label matches any parameter name or label
                            // If the argument label is not specified, use the parameter of the number
                            if let Some(target_param) = function.params.iter().find(|param| {
                                Some(&param.name) == arg.label.as_ref() || param.label == arg.label
                            }) {
                                parsed_arguments.push(FuncCallArg {
                                    name: target_param.name.clone(),
                                    value: tree_expr,
                                });
                            } else {
                                let target_param = &function.params[i];
                                parsed_arguments.push(FuncCallArg {
                                    name: target_param.name.clone(),
                                    value: tree_expr,
                                });
                            }
                        }

                        errors.push(ConstructorError {
                            error_type: ConstructorErrorType::ArgumentNotFound(arg.label.clone()),
                            position: arg.range,
                        });
                    }

                    stack.push((
                        Expression::FuncCall {
                            path: resolved_func_path,
                            args: parsed_arguments,
                        },
                        value_type,
                    ))
                }
                _ => (),
            },
            TypedTokenKind::PrefixOperator(ref symbol) => {
                let (operand, operand_type) = stack.pop().ok_or_else(|| {
                    vec![ConstructorError {
                        error_type: ConstructorErrorType::ArityMismatch(symbol.clone(), 1),
                        position: current_token.range,
                    }]
                })?;

                // Get the operator in order to determine the return type
                let operator = program
                    .get_prefix_func(&operand_type, symbol)
                    .ok_or_else(|| {
                        vec![ConstructorError {
                            error_type: ConstructorErrorType::OperatorNotFound(symbol.clone()),
                            position: current_token.range,
                        }]
                    })?;

                let operator_return_type = operator.return_type.as_ref().ok_or_else(|| {
                    vec![ConstructorError {
                        error_type: ConstructorErrorType::CompilerBug(
                            "Operator return type should have already been determined.".to_string(),
                        ),
                        position: current_token.range,
                    }]
                })?;

                let operator_expr = Expression::PrefixOperator {
                    operand: Box::new(operand),
                    operand_type,
                    return_type: operator_return_type.clone(),
                };

                stack.push((operator_expr, operator_return_type.clone()));
            }
            TypedTokenKind::InfixOperator(ref symbol) => {
                let (rhs, rhs_type) = stack.pop().ok_or_else(|| {
                    vec![ConstructorError {
                        error_type: ConstructorErrorType::ArityMismatch(symbol.clone(), 2),
                        position: current_token.range,
                    }]
                })?;
                let (lhs, lhs_type) = stack.pop().ok_or_else(|| {
                    vec![ConstructorError {
                        error_type: ConstructorErrorType::ArityMismatch(symbol.clone(), 2),
                        position: current_token.range,
                    }]
                })?;

                // Get the operator in order to determine the return type
                let operator = program
                    .get_infix_func(&lhs_type, &rhs_type, symbol)
                    .ok_or_else(|| {
                        vec![ConstructorError {
                            error_type: ConstructorErrorType::OperatorNotFound(symbol.clone()),
                            position: current_token.range,
                        }]
                    })?;
                let operator_return_type = operator.return_type.as_ref().ok_or_else(|| {
                    vec![ConstructorError {
                        error_type: ConstructorErrorType::CompilerBug(
                            "Operator return type should have already been determined.".to_string(),
                        ),
                        position: current_token.range,
                    }]
                })?;

                let operator_expr = Expression::InfixOperator {
                    lhs: Box::new(lhs),
                    lhs_type,
                    rhs: Box::new(rhs),
                    rhs_type,
                    return_type: operator_return_type.clone(),
                };

                stack.push((operator_expr, operator_return_type.clone()));
            }
            _ => {
                return Err(vec![ConstructorError {
                    error_type: ConstructorErrorType::CompilerBug(
                        "Parenthesis should not be in the RPN token list.".to_string(),
                    ),
                    position: current_token.range,
                }]);
            }
        }
    }

    if stack.len() != 1 {
        Err(vec![ConstructorError {
            error_type: ConstructorErrorType::ExprSyntaxError,
            position: Range::zero(),
        }])
    } else {
        let root = stack.pop().unwrap();
        Ok(root.0)
    }
}
