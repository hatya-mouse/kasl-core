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
    ConstructorError, ConstructorErrorType, Function, InputVar, OutputVar, ParserStatementKind,
    Program, StateVar, SymbolPath, SymbolTable,
};

// Collect all symbols from top-level and add them to the symbol table.
pub fn collect_top_level_symbols(
    program: &mut Program,
    symbol_table: &SymbolTable,
) -> Result<(), ConstructorError> {
    for stmt in &symbol_table.vars {
        match &stmt.1.kind {
            ParserStatementKind::Input {
                name,
                value_type: _,
                def_val: _,
                attrs: _,
            } => {
                let input = InputVar {
                    name: name.to_string(),
                    value_type: None,
                    def_val: None,
                    attrs: Vec::new(),
                };
                program.register_input_by_path(input, &SymbolPath::root())?;
            }

            ParserStatementKind::Output {
                name,
                value_type: _,
            } => {
                let output = OutputVar {
                    name: name.to_string(),
                    value_type: None,
                };
                program.register_output_by_path(output, &SymbolPath::root())?;
            }

            ParserStatementKind::State { vars } => {
                for var in vars {
                    let state = StateVar {
                        name: var.name.to_string(),
                        value_type: None,
                        def_val: None,
                    };
                    program.register_state_by_path(state, &SymbolPath::root())?;
                }
            }

            _ => (),
        }
    }

    for stmt in &symbol_table.funcs {
        match &stmt.1.kind {
            ParserStatementKind::FuncDecl {
                required_by,
                name,
                params: _,
                return_type: _,
                body: _,
            } => {
                if required_by.is_some() {
                    return Err(ConstructorError {
                        position: stmt.1.range,
                        error_type: ConstructorErrorType::InvalidRequiredBy,
                    });
                }

                let function = Function {
                    name: name.to_string(),
                    params: Vec::new(),
                    return_type: None,
                    body: Vec::new(),
                    required_by: None,
                };
                program.register_func_by_path(function, &SymbolPath::root())?;
            }

            _ => (),
        }
    }

    Ok(())
}
