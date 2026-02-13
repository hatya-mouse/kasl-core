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
    ConstructorError, ConstructorErrorType, ParserFuncParam, ParserSymbolPath, Program, Range,
    SymbolPath,
};

pub fn resolve_prefix_operator(
    program: &mut Program,
    symbol: &str,
    symbol_path: &SymbolPath,
    params: &[ParserFuncParam],
    return_type: &ParserSymbolPath,
    decl_range: Range,
) -> Result<(), ConstructorError> {
    println!("Resolving prefix operator: {}", symbol);

    if params.len() != 1 {
        return Err(ConstructorError {
            error_type: ConstructorErrorType::InvalidOperatorParams(symbol.to_string()),
            position: decl_range,
        });
    }

    if let Some(value_type) = &params[0].value_type {
        if let Some(operand_type) = program.resolve_type_def_parser_path(&value_type) {
            // If the symbol has a type annotation, use it
            // Get the operator by the retrieved type and the symbokl path
            let operator = match program.get_prefix_func_mut(&operand_type, symbol) {
                Some(func) => func,
                None => {
                    return Err(ConstructorError {
                        error_type: ConstructorErrorType::OperatorNotFound(symbol.to_string()),
                        position: decl_range,
                    });
                }
            };

            operator.operand.value_type = Some(operand_type.clone());

            resolve_prefix_return_type(
                program,
                symbol,
                symbol_path,
                &operand_type,
                return_type,
                decl_range,
            )?;
        }
    }

    Ok(())
}

fn resolve_prefix_return_type(
    program: &mut Program,
    symbol: &str,
    symbol_path: &SymbolPath,
    operand_type: &SymbolPath,
    return_type: &ParserSymbolPath,
    decl_range: Range,
) -> Result<(), ConstructorError> {
    // If the function has a return type, resolve it
    if let Some(return_type_path) = program.resolve_type_def_parser_path(return_type) {
        match program.get_prefix_func_mut(operand_type, symbol) {
            Some(func) => func.return_type = Some(return_type_path),
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                    position: decl_range,
                });
            }
        }
    } else {
        return Err(ConstructorError {
            error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
            position: decl_range,
        });
    }

    Ok(())
}
