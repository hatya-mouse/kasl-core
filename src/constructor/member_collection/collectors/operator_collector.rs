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
    ConstructorError, ConstructorErrorType, FuncParam, Operator, OperatorAssociativity,
    OperatorKind, ParserStatementKind, Program, SymbolPath, SymbolTable,
};

pub fn collect_member_operators(
    program: &mut Program,
    symbol_table: &SymbolTable,
    scope_path: &SymbolPath,
) -> Result<(), ConstructorError> {
    for stmt in &symbol_table.operators {
        match &stmt.1.kind {
            ParserStatementKind::Infix {
                symbol,
                params,
                return_type: _,
                attrs: _,
                body: _,
            } => {
                let param = if let Some(first_param) = params.first() {
                    FuncParam {
                        label: first_param.label.clone(),
                        name: first_param.name.clone(),
                        value_type: None,
                        def_val: None,
                    }
                } else {
                    return Err(ConstructorError {
                        error_type: ConstructorErrorType::InvalidParamForOp,
                        position: stmt.1.range,
                    });
                };

                let operator = Operator {
                    symbol: symbol.clone(),
                    return_type: None,
                    body: Vec::new(),
                    kind: OperatorKind::InfixOperator {
                        another: param,
                        associativity: OperatorAssociativity::Left,
                        precedence: 0,
                    },
                };
                program.register_operator_by_path(operator, scope_path)?;
            }

            ParserStatementKind::Prefix {
                symbol,
                params,
                return_type: _,
                body: _,
            } => {
                if params.len() > 0 {
                    return Err(ConstructorError {
                        error_type: ConstructorErrorType::InvalidParamForOp,
                        position: stmt.1.range,
                    });
                }

                let operator = Operator {
                    symbol: symbol.clone(),
                    return_type: None,
                    body: Vec::new(),
                    kind: OperatorKind::PrefixOperator,
                };
                program.register_operator_by_path(operator, scope_path)?;
            }

            ParserStatementKind::Postfix {
                symbol,
                params,
                return_type: _,
                body: _,
            } => {
                if params.len() > 0 {
                    return Err(ConstructorError {
                        error_type: ConstructorErrorType::InvalidParamForOp,
                        position: stmt.1.range,
                    });
                }

                let operator = Operator {
                    symbol: symbol.clone(),
                    return_type: None,
                    body: Vec::new(),
                    kind: OperatorKind::PostfixOperator,
                };
                program.register_operator_by_path(operator, scope_path)?;
            }

            _ => (),
        }
    }

    Ok(())
}
