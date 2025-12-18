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
    ConstructorError, ConstructorErrorType, ExprToken, ExprTokenKind, LiteralBind, Program,
    SymbolPath, SymbolTable,
};

pub fn collect_token_type(
    program: &Program,
    expr: &[ExprToken],
    symbol_table: &SymbolTable,
) -> Result<Vec<Option<SymbolPath>>, ConstructorError> {
    let mut expr_iter = expr.iter().peekable();
    let mut token_type: Vec<Option<SymbolPath>> = Vec::new();

    while let Some(token) = expr_iter.next() {
        match token.kind {
            ExprTokenKind::IntLiteral(_) => match &program.int_literal_type {
                Some(int_literal_type) => token_type.push(Some(int_literal_type.clone())),
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
                Some(float_literal_type) => token_type.push(Some(float_literal_type.clone())),
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
                Some(bool_literal_type) => token_type.push(Some(bool_literal_type.clone())),
                None => {
                    return Err(ConstructorError {
                        error_type: ConstructorErrorType::MissingLiteralBind(
                            LiteralBind::BoolLiteral,
                        ),
                        position: token.range,
                    });
                }
            },

            ExprTokenKind::Identifier(ref parser_path) => {
                let symbol_type = program.get_symbol_type(&parser_path, symbol_table, token)?;
                token_type.push(Some(symbol_type));
            }

            ExprTokenKind::FuncCall {
                path: ref func_parser_path,
                args: _,
            } => {
                let func_type = program.get_func_type(func_parser_path, symbol_table, token)?;
                token_type.push(Some(func_type));
            }

            ExprTokenKind::Operator(_) => {
                token_type.push(None);
            }

            _ => (),
        }
    }

    Ok(token_type)
}
