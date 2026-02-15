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
    ExprTokenKind, Expression, FuncCallArg, Program, Range, SymbolTable, TypedToken,
    TypedTokenKind,
    error::{ErrorCollector, Phase},
    resolution::expr_inference::ExprTreeBuilder,
};

/// Convert an RPN sequence of TypedToken into an Expression tree.
/// Returns Err on arity errors, unknown symbols, or missing operator implementations.
pub fn build_expr_tree_from_rpn(
    ec: &mut ErrorCollector,
    program: &Program,
    symbol_table: &SymbolTable,
    rpn_tokens: Vec<TypedToken>,
) -> Option<Expression> {
    let mut stack = Vec::new();
    let expr_range = rpn_tokens
        .first()
        .map_or(Range::zero(), |token| token.range);

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
                    let resolved_var_path = match symbol_table.resolve_path(path) {
                        Some(path) => path,
                        None => {
                            ec.var_not_found(
                                current_token.range,
                                Phase::TypeResolution,
                                &path.to_string(),
                            );
                            return None;
                        }
                    };
                    stack.push((Expression::Identifier(resolved_var_path), value_type))
                }
                ExprTokenKind::FuncCall { ref path, args } => {
                    let resolved_func_path = match symbol_table.resolve_path(path) {
                        Some(path) => path,
                        None => {
                            ec.func_not_found(
                                current_token.range,
                                Phase::TypeResolution,
                                &path.to_string(),
                            );
                            return None;
                        }
                    };

                    let function = match program.get_func_by_path(&resolved_func_path) {
                        Some(func) => func,
                        None => {
                            ec.func_not_found(
                                current_token.range,
                                Phase::TypeResolution,
                                &resolved_func_path.to_string(),
                            );
                            return None;
                        }
                    };

                    // Build expression tree for arguments
                    let mut parsed_arguments = Vec::new();
                    for (i, arg) in args.iter().enumerate() {
                        // Check if the function has enough number of arguments
                        if i < function.params.len() {
                            // Build expression tree for an argument
                            let tree_expr = match program.build_expr_tree_from_raw_tokens(
                                ec,
                                &arg.value,
                                symbol_table,
                            ) {
                                Some(expr) => expr,
                                None => continue,
                            };

                            // Check if the argument label matches any parameter name or label
                            // If the argument label is not specified, use the parameter in the index
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
                let (operand, operand_type) = match stack.pop() {
                    Some((operand, operand_type)) => (operand, operand_type),
                    None => {
                        ec.arity_mismatch(current_token.range, Phase::TypeResolution, 1, 0);
                        return None;
                    }
                };

                // Get the operator in order to determine the return type
                let operator = match program.get_prefix_func(&operand_type, symbol) {
                    Some(operator) => operator,
                    None => {
                        ec.operator_not_found(current_token.range, Phase::TypeResolution, symbol);
                        return None;
                    }
                };

                let operator_expr = Expression::PrefixOperator {
                    operand: Box::new(operand),
                    operand_type,
                    return_type: operator.return_type.clone(),
                };

                stack.push((operator_expr, operator.return_type.clone()));
            }
            TypedTokenKind::InfixOperator(ref symbol) => {
                let (rhs, rhs_type) = match stack.pop() {
                    Some((rhs, rhs_type)) => (rhs, rhs_type),
                    None => {
                        ec.arity_mismatch(current_token.range, Phase::TypeResolution, 2, 0);
                        return None;
                    }
                };
                let (lhs, lhs_type) = match stack.pop() {
                    Some((lhs, lhs_type)) => (lhs, lhs_type),
                    None => {
                        ec.arity_mismatch(current_token.range, Phase::TypeResolution, 2, 1);
                        return None;
                    }
                };

                // Get the operator in order to determine the return type
                let operator = match program.get_infix_func(&lhs_type, &rhs_type, symbol) {
                    Some(operator) => operator,
                    None => {
                        ec.operator_not_found(current_token.range, Phase::TypeResolution, symbol);
                        return None;
                    }
                };

                let operator_expr = Expression::InfixOperator {
                    lhs: Box::new(lhs),
                    lhs_type,
                    rhs: Box::new(rhs),
                    rhs_type,
                    return_type: operator.return_type.clone(),
                };

                stack.push((operator_expr, operator.return_type.clone()));
            }
            _ => {
                ec.comp_bug(
                    current_token.range,
                    Phase::TypeResolution,
                    "Parentheses should not be in the RPN token list.",
                );
                return None;
            }
        }
    }

    if stack.len() != 1 {
        ec.invalid_expr_syntax(expr_range, Phase::TypeResolution);
        None
    } else {
        let root = stack.pop().unwrap();
        Some(root.0)
    }
}
