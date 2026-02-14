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
    ConstructorError, ConstructorErrorType, FuncParam, InfixOperator, ParserFuncParam,
    ParserSymbolPath, Program, Range, SymbolPath,
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

    // Get the return type
    let return_type_path = program
        .resolve_type_def_parser_path(return_type)
        .ok_or_else(|| ConstructorError {
            error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
            position: decl_range,
        })?;

    // Once we've got the types, we can get the exact operator
    let infix = InfixOperator {
        symbol: symbol.to_string(),
        lhs: FuncParam {
            label: params[0].label.clone(),
            name: params[0].name.clone(),
            value_type: Some(lhs_type),
            def_val: None,
        },
        rhs: FuncParam {
            label: params[1].label.clone(),
            name: params[1].name.clone(),
            value_type: Some(rhs_type),
            def_val: None,
        },
        return_type: Some(return_type_path),
        body: Vec::new(),
    };
    program.register_infix_func(infix);

    Ok(())
}
