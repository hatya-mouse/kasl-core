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
    FuncParam, Function, Initializer, Operator, OperatorAssociativity, ParserStatement,
    ParserStatementKind, Program, ResolverError, ResolverErrorType, TypeDef, Variable,
};

pub fn collect_members(
    program: &Program,
    stmts: &[ParserStatement],
    type_def: &mut TypeDef,
) -> Result<(), ResolverError> {
    collect_member_variables(program, stmts, type_def)?;
    collect_member_functions(program, stmts, type_def)?;
    collect_member_nests(program, stmts, type_def)?;
    collect_member_operators(program, stmts, type_def)?;

    Ok(())
}

pub fn collect_member_variables(
    program: &Program,
    stmts: &[ParserStatement],
    type_def: &mut TypeDef,
) -> Result<(), ResolverError> {
    for stmt in stmts {
        match &stmt.kind {
            ParserStatementKind::Var {
                required_by,
                name,
                value_type,
                def_val: _,
            } => {
                let resolved_required_by = match required_by {
                    Some(ty) => Some(program.resolve_type(ty)?),
                    None => None,
                };

                type_def.vars.push(Variable {
                    required_by: resolved_required_by,
                    name: name.clone(),
                    value_type: match value_type {
                        Some(ty) => Some(program.resolve_type(ty)?),
                        None => None,
                    },
                    def_val: None,
                });
            }

            _ => (),
        }
    }

    Ok(())
}

pub fn collect_member_functions(
    program: &Program,
    stmts: &[ParserStatement],
    type_def: &mut TypeDef,
) -> Result<(), ResolverError> {
    for stmt in stmts {
        match &stmt.kind {
            ParserStatementKind::FuncDecl {
                required_by,
                name,
                params,
                return_type,
                body: _,
            } => {
                let resolved_required_by = match required_by {
                    Some(ty) => Some(program.resolve_type(ty)?),
                    None => None,
                };
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

                type_def.funcs.push(Function {
                    name: name.to_string(),
                    params: params_result?,
                    return_type: resolved_return_type,
                    body: Vec::new(),
                    required_by: resolved_required_by,
                });
            }

            ParserStatementKind::Init {
                required_by,
                literal_bind,
                params,
                body: _,
            } => {
                let resolved_required_by = match required_by {
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

                type_def.inits.push(Initializer {
                    literal_bind: literal_bind.clone(),
                    params: params_result?,
                    body: Vec::new(),
                    required_by: resolved_required_by,
                });
            }

            _ => (),
        }
    }

    Ok(())
}

pub fn collect_member_nests(
    program: &Program,
    stmts: &[ParserStatement],
    type_def: &mut TypeDef,
) -> Result<(), ResolverError> {
    for stmt in stmts {
        match &stmt.kind {
            ParserStatementKind::StructDecl {
                name,
                inherits,
                body,
            } => {
                let child_type_def = type_def.fine_type_def_mut(name);
                match child_type_def {
                    Some(child_type_def) => {
                        let child_type_inherits: Result<Vec<_>, _> =
                            inherits.iter().map(|ty| program.resolve_type(ty)).collect();
                        child_type_def.inherits = child_type_inherits?;
                        collect_members(program, body, child_type_def)?;
                    }
                    None => {
                        return Err(ResolverError {
                            error_type: ResolverErrorType::TypeNotFound(name.to_string()),
                            offset: 0,
                        });
                    }
                }
            }

            ParserStatementKind::ProtocolDecl {
                name,
                inherits,
                body,
            } => {
                let child_type_def = type_def.fine_type_def_mut(name);
                match child_type_def {
                    Some(child_type_def) => {
                        let child_type_inherits: Result<Vec<_>, _> =
                            inherits.iter().map(|ty| program.resolve_type(ty)).collect();
                        child_type_def.inherits = child_type_inherits?;
                        collect_members(program, body, child_type_def)?;
                    }
                    None => {
                        return Err(ResolverError {
                            error_type: ResolverErrorType::TypeNotFound(name.to_string()),
                            offset: 0,
                        });
                    }
                }
            }

            _ => (),
        }
    }

    Ok(())
}

pub fn collect_member_operators(
    program: &Program,
    stmts: &[ParserStatement],
    type_def: &mut TypeDef,
) -> Result<(), ResolverError> {
    for stmt in stmts {
        match &stmt.kind {
            ParserStatementKind::Infix {
                symbol,
                params,
                return_type,
                attrs: _,
                body: _,
            } => {
                let resolved_return_type = program.resolve_type(return_type)?;

                // TODO: params processing
                let param = if let Some(first_param) = params.first() {
                    FuncParam {
                        label: first_param.label.clone(),
                        name: first_param.name.clone(),
                        value_type: match &first_param.value_type {
                            Some(ty) => Some(program.resolve_type(ty)?),
                            None => None,
                        },
                        def_val: None,
                    }
                } else {
                    return Err(ResolverError {
                        error_type: ResolverErrorType::ExpectType,
                        offset: 0,
                    });
                };

                // TODO: parse attrs for associativity and precedence
                type_def.operators.push(Operator::InfixOperator {
                    symbol: symbol.clone(),
                    another: param,
                    return_type: resolved_return_type,
                    associativity: OperatorAssociativity::Left,
                    precedence: 0,
                    body: Vec::new(),
                });
            }

            ParserStatementKind::Prefix {
                symbol,
                params,
                return_type,
                body: _,
            } => {
                let resolved_return_type = program.resolve_type(return_type)?;

                // TODO: params processing
                let param = if let Some(first_param) = params.first() {
                    FuncParam {
                        label: first_param.label.clone(),
                        name: first_param.name.clone(),
                        value_type: match &first_param.value_type {
                            Some(ty) => Some(program.resolve_type(ty)?),
                            None => None,
                        },
                        def_val: None,
                    }
                } else {
                    return Err(ResolverError {
                        error_type: ResolverErrorType::ExpectType,
                        offset: 0,
                    });
                };

                type_def.operators.push(Operator::PrefixOperator {
                    symbol: symbol.clone(),
                    another: param,
                    return_type: resolved_return_type,
                    body: Vec::new(), // TODO: implement body
                });
            }

            ParserStatementKind::Postfix {
                symbol,
                params,
                return_type,
                body: _,
            } => {
                let resolved_return_type = program.resolve_type(return_type)?;

                // TODO: params processing
                let param = if let Some(first_param) = params.first() {
                    FuncParam {
                        label: first_param.label.clone(),
                        name: first_param.name.clone(),
                        value_type: match &first_param.value_type {
                            Some(ty) => Some(program.resolve_type(ty)?),
                            None => None,
                        },
                        def_val: None,
                    }
                } else {
                    return Err(ResolverError {
                        error_type: ResolverErrorType::ExpectType,
                        offset: 0,
                    });
                };

                type_def.operators.push(Operator::PostfixOperator {
                    symbol: symbol.clone(),
                    another: param,
                    return_type: resolved_return_type,
                    body: Vec::new(), // TODO: implement body
                });
            }

            _ => (),
        }
    }

    Ok(())
}
