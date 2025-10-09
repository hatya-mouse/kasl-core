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
    Function, InputVar, OutputVar, ParserStatement, ParserStatementKind, Program, ResolverError,
    ResolverErrorType, StateVar, SymbolTable,
};

//
pub fn collect_top_level_symbols(
    program: &mut Program,
    symbol_table: &mut SymbolTable,
    stmts: &[ParserStatement],
) -> Result<(), ResolverError> {
    for stmt in stmts {
        match &stmt.kind {
            ParserStatementKind::Input {
                name,
                value_type: _,
                def_val: _,
                attrs: _,
            } => {
                program.inputs.push(InputVar {
                    name: name.to_string(),
                    value_type: None,
                    def_val: None,
                    attrs: Vec::new(),
                });
            }

            ParserStatementKind::Output {
                name,
                value_type: _,
            } => program.outputs.push(OutputVar {
                name: name.to_string(),
                value_type: None,
            }),
            ParserStatementKind::State { vars } => {
                for var in vars {
                    program.states.push(StateVar {
                        name: var.name.to_string(),
                        value_type: None,
                        def_val: None,
                    });
                }
            }

            ParserStatementKind::FuncDecl {
                required_by,
                name,
                params: _,
                return_type: _,
                body: _,
            } => {
                if required_by.is_some() {
                    return Err(ResolverError {
                        position: stmt.range,
                        error_type: ResolverErrorType::InvalidRequiredBy,
                    });
                }

                program.funcs.push(Function {
                    name: name.to_string(),
                    params: Vec::new(),
                    return_type: None,
                    body: Vec::new(),
                    required_by: None,
                })
            }

            _ => (),
        }
    }

    Ok(())
}
