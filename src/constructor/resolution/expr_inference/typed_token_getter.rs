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
    ConstructorError, ConstructorErrorType, ExprToken, ExprTokenKind, LiteralBind, Program, Range,
    SymbolPath, SymbolTable,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedToken {
    pub kind: TypedTokenKind,
    pub range: Range,
}

impl TypedToken {
    pub fn new(kind: TypedTokenKind, range: Range) -> Self {
        TypedToken { kind, range }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypedTokenKind {
    Value(SymbolPath), // The type of the value
    PrefixOperator(String),
    InfixOperator(String),
    LParen,
    RParen,
}

/// Infer the type of each token in the expression and convert them to TypedTokens.
pub fn get_typed_tokens(
    program: &Program,
    expr: &[ExprToken],
    symbol_table: &SymbolTable,
) -> Result<Vec<TypedToken>, ConstructorError> {
    let mut expr_iter = expr.iter().peekable();
    let mut result: Vec<TypedToken> = Vec::new();

    while let Some(token) = expr_iter.next() {
        match &token.kind {
            ExprTokenKind::IntLiteral(_) => match &program.int_literal_type {
                Some(int_literal_type) => result.push(TypedToken::new(
                    TypedTokenKind::Value(int_literal_type.clone()),
                    token.range.clone(),
                )),
                None => {
                    return Err(ConstructorError {
                        error_type: ConstructorErrorType::MissingLiteralBind(
                            LiteralBind::IntLiteral,
                        ),
                        position: token.range,
                    });
                }
            },

            ExprTokenKind::FloatLiteral(_) => match &program.float_literal_type {
                Some(float_literal_type) => result.push(TypedToken::new(
                    TypedTokenKind::Value(float_literal_type.clone()),
                    token.range.clone(),
                )),
                None => {
                    return Err(ConstructorError {
                        error_type: ConstructorErrorType::MissingLiteralBind(
                            LiteralBind::FloatLiteral,
                        ),
                        position: token.range,
                    });
                }
            },

            ExprTokenKind::BoolLiteral(_) => match &program.bool_literal_type {
                Some(bool_literal_type) => result.push(TypedToken::new(
                    TypedTokenKind::Value(bool_literal_type.clone()),
                    token.range.clone(),
                )),
                None => {
                    return Err(ConstructorError {
                        error_type: ConstructorErrorType::MissingLiteralBind(
                            LiteralBind::BoolLiteral,
                        ),
                        position: token.range,
                    });
                }
            },

            ExprTokenKind::Identifier(parser_path) => {
                let symbol_type = program.get_symbol_type(&parser_path, symbol_table, token)?;
                result.push(TypedToken::new(
                    TypedTokenKind::Value(symbol_type),
                    token.range.clone(),
                ));
            }

            ExprTokenKind::FuncCall {
                path: func_parser_path,
                args: _,
            } => {
                let func_type = program.get_func_type(func_parser_path, symbol_table, token)?;
                result.push(TypedToken::new(
                    TypedTokenKind::Value(func_type),
                    token.range.clone(),
                ));
            }

            ExprTokenKind::Operator(operator_symbol) => {
                let last_token = result.last();
                let operator_token =
                    handle_operator_resolution(operator_symbol, token.range.clone(), last_token);
                result.push(operator_token);
            }

            ExprTokenKind::LParen => {
                result.push(TypedToken::new(TypedTokenKind::LParen, token.range.clone()))
            }

            ExprTokenKind::RParen => {
                result.push(TypedToken::new(TypedTokenKind::RParen, token.range.clone()))
            }
        }
    }

    Ok(result)
}

fn handle_operator_resolution(
    operator_symbol: &String,
    operator_range: Range,
    last_token: Option<&TypedToken>,
) -> TypedToken {
    // Whether the operator is infix or prefix can be determined by the last token
    let is_infix = match last_token {
        Some(unwrapped_token) => match unwrapped_token.kind {
            TypedTokenKind::LParen | TypedTokenKind::RParen => false,
            _ => true,
        },
        None => false,
    };

    if is_infix {
        return TypedToken::new(
            TypedTokenKind::InfixOperator(operator_symbol.clone()),
            operator_range,
        );
    } else {
        return TypedToken::new(
            TypedTokenKind::PrefixOperator(operator_symbol.clone()),
            operator_range,
        );
    }
}
