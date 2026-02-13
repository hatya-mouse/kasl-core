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

pub fn resolve_infix_func(
    program: &mut Program,
    symbol: &str,
    symbol_path: &SymbolPath,
    params: &[ParserFuncParam],
    return_type: &ParserSymbolPath,
    decl_range: Range,
) -> Result<(), ConstructorError> {
    if params.len() != 2 {
        return Err(ConstructorError {
            error_type: ConstructorErrorType::InvalidOperatorParams(symbol.to_string()),
            position: decl_range,
        });
    }

    let mut types = Vec::new();
    for param in params {
        if let Some(value_type) = &param.value_type {
            if let Some(type_symbol_path) = program.resolve_type_def_parser_path(&value_type) {
                // If the symbol has a type annotation, use it
                types.push(type_symbol_path.clone());
            } else {
                // If the type annotation is not found, push an error
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                    position: decl_range,
                });
            }
        };
    }

    let lhs_type = types[0].clone();
    let rhs_type = types[1].clone();

    // Once we've got the types, we can get the exact operator
    let operator = match program.get_infix_func_mut(&lhs_type, &rhs_type, symbol) {
        Some(func) => func,
        None => {
            return Err(ConstructorError {
                error_type: ConstructorErrorType::OperatorNotFound(symbol.to_string()),
                position: decl_range,
            });
        }
    };

    operator.lhs.value_type = Some(lhs_type.clone());
    operator.rhs.value_type = Some(rhs_type.clone());

    resolve_infix_return_type(
        program,
        symbol,
        symbol_path,
        &lhs_type,
        &rhs_type,
        return_type,
        decl_range,
    )?;

    Ok(())
}

fn resolve_infix_return_type(
    program: &mut Program,
    symbol: &str,
    symbol_path: &SymbolPath,
    lhs_type: &SymbolPath,
    rhs_type: &SymbolPath,
    return_type: &ParserSymbolPath,
    decl_range: Range,
) -> Result<(), ConstructorError> {
    // If the function has a return type, resolve it
    if let Some(return_type_path) = program.resolve_type_def_parser_path(return_type) {
        match program.get_infix_func_mut(lhs_type, rhs_type, symbol) {
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
