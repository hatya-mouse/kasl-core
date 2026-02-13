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
    ConstructorError, ConstructorErrorType, FuncParam, Function, InfixOperator, InputVar,
    OutputVar, ParserOperatorType, ParserStatementKind, PrefixOperator, Program, StateVar,
    SymbolTable, member_collection::collectors::construct_func_params,
};

// Collect all symbols from top-level and add them to the symbol table.
pub fn collect_top_level_symbols(
    program: &mut Program,
    symbol_table: &SymbolTable,
) -> Result<(), ConstructorError> {
    for stmt in &symbol_table.inputs {
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
                program.register_input(input);
            }

            _ => (),
        }
    }

    for stmt in &symbol_table.outputs {
        match &stmt.1.kind {
            ParserStatementKind::Output {
                name,
                value_type: _,
                def_val: _,
            } => {
                let output = OutputVar {
                    name: name.to_string(),
                    value_type: None,
                    def_val: None,
                };
                program.register_output(output);
            }

            _ => (),
        }
    }

    for stmt in &symbol_table.states {
        match &stmt.1.kind {
            ParserStatementKind::State { vars } => {
                for var in vars {
                    let state = StateVar {
                        name: var.name.to_string(),
                        value_type: None,
                        def_val: None,
                    };
                    program.register_state(state);
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
                params,
                return_type: _,
                body: _,
            } => {
                if required_by.is_some() {
                    return Err(ConstructorError {
                        position: stmt.1.range,
                        error_type: ConstructorErrorType::InvalidRequiredBy,
                    });
                }

                let func_params = construct_func_params(params);
                let function = Function {
                    name: name.to_string(),
                    params: func_params,
                    return_type: None,
                    body: Vec::new(),
                    required_by: None,
                };
                program.register_func(function);
            }

            _ => (),
        }
    }

    for stmt in &symbol_table.infix_defines {
        match &stmt.1.kind {
            ParserStatementKind::InfixDefine {
                symbol,
                infix_properties,
            } => program.register_infix_operator(symbol.to_string(), infix_properties.clone()),
            _ => (),
        }
    }

    for stmt in &symbol_table.prefix_defines {
        match &stmt.1.kind {
            ParserStatementKind::PrefixDefine { symbol } => {
                program.register_prefix_operator(symbol.to_string())
            }
            _ => (),
        }
    }

    for stmt in &symbol_table.infix_funcs {
        match &stmt.1.kind {
            ParserStatementKind::OperatorFunc {
                op_type,
                symbol,
                params,
                return_type: _,
                body: _,
            } => match op_type {
                ParserOperatorType::Infix => {
                    if params.len() != 2 {
                        return Err(ConstructorError {
                            error_type: ConstructorErrorType::InvalidOperatorParams(symbol.clone()),
                            position: stmt.1.range,
                        });
                    }

                    let infix = InfixOperator {
                        symbol: symbol.to_string(),
                        lhs: FuncParam {
                            label: params[0].label.clone(),
                            name: params[0].name.clone(),
                            value_type: None,
                            def_val: None,
                        },
                        rhs: FuncParam {
                            label: params[1].label.clone(),
                            name: params[1].name.clone(),
                            value_type: None,
                            def_val: None,
                        },
                        return_type: None,
                        body: Vec::new(),
                    };
                    program.register_infix_func(infix);
                }
                _ => (),
            },
            _ => (),
        }
    }

    for stmt in &symbol_table.prefix_funcs {
        match &stmt.1.kind {
            ParserStatementKind::OperatorFunc {
                op_type,
                symbol,
                params,
                return_type: _,
                body: _,
            } => match op_type {
                ParserOperatorType::Prefix => {
                    if params.len() != 1 {
                        return Err(ConstructorError {
                            error_type: ConstructorErrorType::InvalidOperatorParams(symbol.clone()),
                            position: stmt.1.range,
                        });
                    }

                    let prefix = PrefixOperator {
                        symbol: symbol.to_string(),
                        operand: FuncParam {
                            label: params[0].label.clone(),
                            name: params[0].name.clone(),
                            value_type: None,
                            def_val: None,
                        },
                        return_type: None,
                        body: Vec::new(),
                    };
                    program.register_prefix_func(prefix);
                }
                _ => (),
            },
            _ => (),
        }
    }

    Ok(())
}
