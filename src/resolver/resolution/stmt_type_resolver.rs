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
    FuncParam, ParserStatement, ParserStatementKind, Program, ResolverError, ResolverErrorType,
    resolution::infer_expr_type,
};

// Construct a resolving tree for functions and variables inside structs or protocols.
pub fn construct_resolving_tree_struct(
    program: &mut Program,
    statements: &[ParserStatement],
) -> Result<(), ResolverError> {
    for stmt in statements {
        match &stmt.kind {
            ParserStatementKind::StructDecl {
                name,
                inherits,
                body,
            }
            | ParserStatementKind::ProtocolDecl {
                name,
                inherits,
                body,
            } => {
                // Loop through the body to resolve types.
                for stmt in body {
                    match &stmt.kind {
                        ParserStatementKind::Input { name, value_type, def_val, attrs }

                        _ => (),
                    }
                }
            }

            _ => (),
        }
    }

    Ok(())
}

/// Resolve types.
pub fn resolve_types(
    program: &mut Program,
    statements: &[ParserStatement],
) -> Result<(), ResolverError> {
    for stmt in statements {
        match &stmt.kind {
            ParserStatementKind::FuncDecl {
                body: _,
                required_by,
                name,
                params,
                return_type,
            } => {
                // Resolve types.
                let r_required_by = match required_by {
                    Some(ty) => Some(program.resolve_type(ty)?),
                    None => None,
                };

                let r_return_type = match return_type {
                    Some(ty) => Some(program.resolve_type(ty)?),
                    None => None,
                };

                let r_params: Result<Vec<FuncParam>, ResolverError> = params
                    .iter()
                    .map(|p| {
                        let r_param_type = match &p.value_type {
                            Some(param_ty) => Some(program.resolve_type(&param_ty)?),
                            // Infer type
                            None => match &p.def_val {
                                Some(expr) => Some(infer_expr_type(program, &expr)?),
                                None => {
                                    return Err(ResolverError {
                                        error_type: ResolverErrorType::AmbiguousDeclaration(
                                            p.name.clone(),
                                        ),
                                        position: p.range,
                                    });
                                }
                            },
                        };

                        return Ok(FuncParam {
                            label: p.label.clone(),
                            name: p.name.clone(),
                            value_type: r_param_type,
                            def_val: None,
                        });
                    })
                    .collect();

                // Get the mutable refernce to the function.
                let func = match program.find_func_mut(name) {
                    Some(func) => func,
                    None => {
                        // Functions should've been parsed within the symbol_collection phase
                        panic!(
                            "Function {} not found in the program which should already be parsed.",
                            name
                        );
                    }
                };

                // Set the resolved types.
                func.required_by = r_required_by;
                func.return_type = r_return_type;
                func.params = r_params?;
            }

            ParserStatementKind::Input {
                name,
                value_type,
                def_val,
                attrs: _,
            } => {
                // Resolve types.
                let r_value_type = match value_type {
                    Some(ty) => Some(program.resolve_type(ty)?),
                    None => match def_val {
                        Some(expr) => Some(infer_expr_type(program, &expr)?),
                        None => {
                            return Err(ResolverError {
                                error_type: ResolverErrorType::AmbiguousDeclaration(name.clone()),
                                position: stmt.range,
                            });
                        }
                    },
                };

                // Get the mutable refernce to the input.
                let input = match program.find_input_mut(name) {
                    Some(input) => input,
                    None => {
                        // Inputs should've been parsed within the symbol_collection phase
                        panic!(
                            "Input {} not found in the program which should already be parsed.",
                            name
                        );
                    }
                };

                // Set the resolved types.
                input.value_type = r_value_type;
            }

            _ => (),
        }
    }

    Ok(())
}
