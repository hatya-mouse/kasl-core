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
    ConstructorError, ConstructorErrorType, ExprToken, ParserSymbolPath, Program, SymbolPath,
    SymbolPathComponent, SymbolTable,
};

impl Program {
    pub fn get_symbol_type(
        &self,
        parser_path: &ParserSymbolPath,
        symbol_table: &SymbolTable,
        token: &ExprToken,
    ) -> Result<SymbolPath, ConstructorError> {
        let symbol_path = match symbol_table.resolve_path(parser_path) {
            Some(path) => path,
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::SymbolNotFound(None),
                    position: token.range,
                });
            }
        };

        match symbol_path.components.last() {
            Some(last_component) => match last_component {
                SymbolPathComponent::InputVar(name) => {
                    // The path has been resolved so we can safely unwrap the input variable
                    let input_var = self.get_input(name).unwrap();
                    input_var.value_type.clone().ok_or(ConstructorError {
                        error_type: ConstructorErrorType::SymbolNotFound(Some(symbol_path.clone())),
                        position: token.range,
                    })
                }
                SymbolPathComponent::OutputVar(name) => {
                    let output_var = self.get_output(name).unwrap();
                    output_var.value_type.clone().ok_or(ConstructorError {
                        error_type: ConstructorErrorType::SymbolNotFound(Some(symbol_path.clone())),
                        position: token.range,
                    })
                }
                SymbolPathComponent::StateVar(name) => {
                    let state_var = self.get_state(name).unwrap();
                    state_var.value_type.clone().ok_or(ConstructorError {
                        error_type: ConstructorErrorType::SymbolNotFound(Some(symbol_path.clone())),
                        position: token.range,
                    })
                }
                SymbolPathComponent::Var(_) => {
                    let scope_var = self.get_var_by_path(&symbol_path).unwrap();
                    scope_var.value_type.clone().ok_or(ConstructorError {
                        error_type: ConstructorErrorType::SymbolNotFound(Some(symbol_path.clone())),
                        position: token.range,
                    })
                }
                _ => {
                    return Err(ConstructorError {
                        error_type: ConstructorErrorType::SymbolNotFound(Some(symbol_path.clone())),
                        position: token.range,
                    });
                }
            },
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::SymbolNotFound(Some(symbol_path.clone())),
                    position: token.range,
                });
            }
        }
    }

    pub fn get_func_type(
        &self,
        parser_path: &ParserSymbolPath,
        symbol_table: &SymbolTable,
        token: &ExprToken,
    ) -> Result<SymbolPath, ConstructorError> {
        let func_symbol_path = match symbol_table.resolve_path(parser_path) {
            Some(path) => path,
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::SymbolNotFound(None),
                    position: token.range,
                });
            }
        };
        let func = match self.get_func_by_path(&func_symbol_path) {
            Some(func) => func,
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::SymbolNotFound(Some(
                        func_symbol_path.clone(),
                    )),
                    position: token.range,
                });
            }
        };

        func.return_type.clone().ok_or(ConstructorError {
            error_type: ConstructorErrorType::NoReturnFunctionInExpr(func_symbol_path),
            position: token.range,
        })
    }
}
