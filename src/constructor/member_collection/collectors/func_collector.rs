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
    FuncParam, Function, Initializer, LiteralBind, ParserStatementKind, Program, SymbolPath,
    SymbolTable, error::ErrorCollector, member_collection::collectors::construct_func_params,
};

pub fn collect_member_functions(
    ec: &mut ErrorCollector,
    program: &mut Program,
    symbol_table: &SymbolTable,
    scope_path: &SymbolPath,
) {
    for stmt in &symbol_table.funcs {
        match &stmt.1.kind {
            ParserStatementKind::FuncDecl {
                required_by: _,
                name,
                params,
                return_type: _,
                body: _,
            } => {
                let func_params = construct_func_params(params);
                let function = Function {
                    name: name.to_string(),
                    params: func_params,
                    return_type: None,
                    body: Vec::new(),
                    required_by: None,
                };
                program.register_func_by_path(ec, function, scope_path, stmt.1.range);
            }

            _ => (),
        }
    }

    for stmt in &symbol_table.inits {
        match &stmt.kind {
            ParserStatementKind::Init {
                required_by: _,
                literal_bind,
                params,
                body: _,
            } => {
                let params_result = params
                    .iter()
                    .map(|param| FuncParam {
                        label: param.label.clone(),
                        name: param.name.clone(),
                        value_type: None,
                        def_val: None,
                    })
                    .collect();

                let initializer = Initializer {
                    literal_bind: literal_bind.clone(),
                    params: params_result,
                    body: Vec::new(),
                    required_by: None,
                };

                if let Some(literal_bind) = literal_bind {
                    match literal_bind {
                        LiteralBind::IntLiteral => {
                            program.set_int_literal(ec, scope_path.clone(), stmt.range)
                        }
                        LiteralBind::FloatLiteral => {
                            program.set_float_literal(ec, scope_path.clone(), stmt.range)
                        }
                        LiteralBind::BoolLiteral => {
                            program.set_bool_literal(ec, scope_path.clone(), stmt.range)
                        }
                    }
                }

                program.register_init_by_path(ec, initializer, scope_path, stmt.range);
            }

            _ => (),
        }
    }
}
