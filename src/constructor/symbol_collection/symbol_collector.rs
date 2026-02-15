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
    Function, InputVar, OutputVar, ParserStatementKind, Program, StateVar, SymbolTable,
    error::{ErrorCollector, Ph},
    member_collection::collectors::construct_func_params,
};

// Collect all symbols from top-level and add them to the symbol table.
pub fn collect_top_level_symbols(
    ec: &mut ErrorCollector,
    program: &mut Program,
    symbol_table: &SymbolTable,
) {
    for stmt in &symbol_table.inputs {
        if let ParserStatementKind::Input {
                name,
                value_type: _,
                def_val: _,
                attrs: _,
            } = &stmt.1.kind {
            let input = InputVar {
                name: name.to_string(),
                value_type: None,
                def_val: None,
                attrs: Vec::new(),
            };
            program.register_input(input);
        }
    }

    for stmt in &symbol_table.outputs {
        if let ParserStatementKind::Output {
                name,
                value_type: _,
                def_val: _,
            } = &stmt.1.kind {
            let output = OutputVar {
                name: name.to_string(),
                value_type: None,
                def_val: None,
            };
            program.register_output(output);
        }
    }

    for stmt in &symbol_table.states {
        if let ParserStatementKind::State { vars } = &stmt.1.kind {
            for var in vars {
                let state = StateVar {
                    name: var.name.to_string(),
                    value_type: None,
                    def_val: None,
                };
                program.register_state(state);
            }
        }
    }

    for stmt in &symbol_table.funcs {
        if let ParserStatementKind::FuncDecl {
                required_by,
                name,
                params,
                return_type: _,
                body: _,
            } = &stmt.1.kind {
            if required_by.is_some() {
                ec.req_by_outside_type(stmt.1.range, Ph::TopLevelCollection);
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
    }

    for stmt in &symbol_table.infix_defines {
        if let ParserStatementKind::InfixDefine {
                symbol,
                infix_properties,
            } = &stmt.1.kind { program.register_infix_operator(symbol.to_string(), infix_properties.clone()) }
    }

    for stmt in &symbol_table.prefix_defines {
        if let ParserStatementKind::PrefixDefine { symbol } = &stmt.1.kind {
            program.register_prefix_operator(symbol.to_string())
        }
    }
}
