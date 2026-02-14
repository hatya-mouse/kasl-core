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
    ConstructorError, ConstructorErrorType, FuncParam, ParserFuncParam, ParserSymbolPath,
    PrefixOperator, Program, Range, SymbolPath,
};

pub fn resolve_prefix_operator(
    program: &mut Program,
    symbol: &str,
    symbol_path: &SymbolPath,
    params: &[ParserFuncParam],
    return_type: &ParserSymbolPath,
    decl_range: Range,
) -> Result<(), ConstructorError> {
    if params.len() != 1 {
        return Err(ConstructorError {
            error_type: ConstructorErrorType::InvalidOperatorParams(symbol.to_string()),
            position: decl_range,
        });
    }

    if let Some(value_type) = &params[0].value_type {
        // Get the operand type path
        let operand_type = program
            .resolve_type_def_parser_path(&value_type)
            .ok_or_else(|| ConstructorError {
                error_type: ConstructorErrorType::SymbolNotFound(None),
                position: decl_range,
            })?;

        // Get the return type path
        let return_type_path = program
            .resolve_type_def_parser_path(return_type)
            .ok_or_else(|| ConstructorError {
                error_type: ConstructorErrorType::CannotInferType(symbol_path.clone()),
                position: decl_range,
            })?;

        // If the symbol has a type annotation, use it
        let prefix = PrefixOperator {
            symbol: symbol.to_string(),
            operand: FuncParam {
                label: params[0].label.clone(),
                name: params[0].name.clone(),
                value_type: Some(operand_type),
                def_val: None,
            },
            return_type: Some(return_type_path),
            body: Vec::new(),
        };
        program.register_prefix_func(prefix);
    }

    Ok(())
}
