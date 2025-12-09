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
    SymbolPath, symbol_path,
};

pub trait ExprTypeInference {
    fn infer_expr_type(&self, expr: &[ExprToken]) -> Result<SymbolPath, ConstructorError>;
}

impl ExprTypeInference for Program {
    fn infer_expr_type(&self, expr: &[ExprToken]) -> Result<SymbolPath, ConstructorError> {
        let mut expr_iter = expr.iter().peekable();
        let mut last_type: Option<SymbolPath> = None;

        while let Some(token) = expr_iter.next() {
            match token.kind {
                ExprTokenKind::IntLiteral(_) => match &self.int_literal_type {
                    Some(int_literal_type) => last_type = Some(int_literal_type.clone()),
                    None => {
                        return Err(ConstructorError {
                            error_type: ConstructorErrorType::MissingLiteralBind(
                                LiteralBind::IntLiteral,
                            ),
                            position: token.range,
                        });
                    }
                },

                ExprTokenKind::FloatLiteral(_) => match &self.float_literal_type {
                    Some(float_literal_type) => last_type = Some(float_literal_type.clone()),
                    None => {
                        return Err(ConstructorError {
                            error_type: ConstructorErrorType::MissingLiteralBind(
                                LiteralBind::FloatLiteral,
                            ),
                            position: token.range,
                        });
                    }
                },

                ExprTokenKind::BoolLiteral(_) => match &self.bool_literal_type {
                    Some(bool_literal_type) => last_type = Some(bool_literal_type.clone()),
                    None => {
                        return Err(ConstructorError {
                            error_type: ConstructorErrorType::MissingLiteralBind(
                                LiteralBind::BoolLiteral,
                            ),
                            position: token.range,
                        });
                    }
                },

                _ => (),
            }
        }

        Ok(symbol_path![])
    }
}
