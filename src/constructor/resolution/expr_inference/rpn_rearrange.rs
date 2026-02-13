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
    ConstructorError, ConstructorErrorType, OperatorAssociativity, Program, TypedToken,
    TypedTokenKind,
};

pub fn rearrange_tokens_to_rpn(
    program: &Program,
    tokens: Vec<TypedToken>,
) -> Result<Vec<TypedToken>, ConstructorError> {
    let mut output_queue: Vec<TypedToken> = Vec::new();
    let mut operator_stack: Vec<TypedToken> = Vec::new();

    for current_token in tokens.into_iter() {
        match current_token.kind {
            TypedTokenKind::Value {
                expr_token: _,
                value_type: _,
            } => output_queue.push(current_token),
            TypedTokenKind::PrefixOperator(_) => operator_stack.push(current_token),
            TypedTokenKind::InfixOperator(ref current_op_symbol) => {
                // Get the precedence and associativity of the current operator
                let current_props = match program.get_infix_operator(current_op_symbol) {
                    Some(props) => props,
                    None => {
                        return Err(ConstructorError {
                            error_type: ConstructorErrorType::OperatorNotFound(
                                current_op_symbol.clone(),
                            ),
                            position: current_token.range,
                        });
                    }
                };

                while let Some(top_token) = operator_stack.last() {
                    match top_token.kind {
                        TypedTokenKind::PrefixOperator(_) => {
                            // Place the unary operator after its operand
                            output_queue.push(operator_stack.pop().unwrap())
                        }
                        TypedTokenKind::InfixOperator(ref top_op_symbol) => {
                            // Get the precedence and associativity of the operator from the stack
                            let top_props = match program.get_infix_operator(top_op_symbol) {
                                Some(props) => props,
                                None => {
                                    return Err(ConstructorError {
                                        error_type: ConstructorErrorType::OperatorNotFound(
                                            top_op_symbol.clone(),
                                        ),
                                        position: top_token.range,
                                    });
                                }
                            };

                            if top_props.precedence > current_props.precedence
                                || (top_props.precedence == current_props.precedence
                                    && current_props.associativity == OperatorAssociativity::Left)
                            {
                                output_queue.push(operator_stack.pop().unwrap());
                            } else {
                                break;
                            }
                        }
                        _ => break,
                    }
                }

                if current_props.associativity == OperatorAssociativity::None {
                    if let Some(top_token) = operator_stack.last() {
                        if let TypedTokenKind::InfixOperator(ref top_op_symbol) = top_token.kind {
                            // Get the precedence and associativity of the operator from the stack
                            let top_props = match program.get_infix_operator(top_op_symbol) {
                                Some(props) => props,
                                None => {
                                    return Err(ConstructorError {
                                        error_type: ConstructorErrorType::OperatorNotFound(
                                            top_op_symbol.clone(),
                                        ),
                                        position: top_token.range,
                                    });
                                }
                            };

                            // If the top operator in the stack has the same precedence as the current operator,
                            // it means that the current operator is chained which is illegal when the associativity is set to None.
                            if top_props.precedence == current_props.precedence {
                                return Err(ConstructorError {
                                    error_type: ConstructorErrorType::OperatorCannotBeChained(
                                        top_op_symbol.clone(),
                                    ),
                                    position: current_token.range,
                                });
                            }
                        }
                    }
                }

                operator_stack.push(current_token);
            }
            TypedTokenKind::LParen => operator_stack.push(current_token),
            TypedTokenKind::RParen => {
                let mut has_l_paren_found = false;
                while let Some(token) = operator_stack.pop() {
                    if token.kind == TypedTokenKind::LParen {
                        has_l_paren_found = true;
                        break;
                    } else {
                        output_queue.push(token);
                    }
                }

                if !has_l_paren_found {
                    return Err(ConstructorError {
                        error_type: ConstructorErrorType::UnmatchedParentheses,
                        position: current_token.range,
                    });
                }
            }
        }
    }

    while let Some(top_token) = operator_stack.pop() {
        if top_token.kind == TypedTokenKind::LParen || top_token.kind == TypedTokenKind::RParen {
            return Err(ConstructorError {
                error_type: ConstructorErrorType::UnmatchedParentheses,
                position: top_token.range,
            });
        }
        output_queue.push(top_token);
    }

    Ok(output_queue)
}
