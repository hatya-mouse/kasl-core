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
    FuncParam, Function, InputVar, OutputVar, ParserStatement, ParserStatementKind, Program,
    ResolverError, ResolverErrorType, StateVar,
};

pub fn collect_top_level_symbols(
    program: &mut Program,
    stmts: &[ParserStatement],
) -> Result<(), ResolverError> {
    for stmt in stmts {
        match &stmt.kind {
            ParserStatementKind::Input {
                name,
                value_type,
                def_val: _,
                attrs: _,
            } => {
                let resolved_type = match value_type {
                    Some(ty) => Some(program.resolve_type(ty)?),
                    None => None,
                };
                program.inputs.push(InputVar {
                    name: name.to_string(),
                    value_type: resolved_type,
                    def_val: None,
                    attrs: Vec::new(),
                });
            }
            ParserStatementKind::Output { name, value_type } => {
                let resolved_type = program.resolve_type(value_type)?;
                program.outputs.push(OutputVar {
                    name: name.to_string(),
                    value_type: resolved_type,
                })
            }
            ParserStatementKind::State { vars } => {
                for var in vars {
                    let resolved_type = match var.value_type {
                        Some(ref ty) => Some(program.resolve_type(ty)?),
                        None => None,
                    };
                    program.states.push(StateVar {
                        name: var.name.to_string(),
                        value_type: resolved_type,
                        def_val: None,
                    });
                }
            }
            ParserStatementKind::FuncDecl {
                required_by,
                name,
                params,
                return_type,
                body: _,
            } => {
                if required_by.is_some() {
                    return Err(ResolverError {
                        offset: stmt.start,
                        error_type: ResolverErrorType::InvalidRequiredBy,
                    });
                }

                let resolved_return_type = match return_type {
                    Some(ty) => Some(program.resolve_type(ty)?),
                    None => None,
                };

                let params_result: Result<Vec<_>, _> = params
                    .iter()
                    .map(|param| {
                        Ok(FuncParam {
                            label: param.label.clone(),
                            name: param.name.clone(),
                            value_type: match param.value_type {
                                Some(ref ty) => Some(program.resolve_type(ty)?),
                                None => None,
                            },
                            def_val: None,
                        })
                    })
                    .collect();

                program.funcs.push(Function {
                    name: name.to_string(),
                    params: params_result?,
                    return_type: resolved_return_type,
                    body: Vec::new(),
                    required_by: None,
                })
            }
            _ => (),
        }
    }

    Ok(())
}
