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
    ConstructorError, FuncParam, Function, Initializer, ParserStatementKind, SymbolTable, TypeDef,
};

pub fn collect_member_functions(
    symbol_table: &SymbolTable,
    type_def: &mut TypeDef,
) -> Result<(), ConstructorError> {
    for stmt in &symbol_table.funcs {
        match &stmt.1.kind {
            ParserStatementKind::FuncDecl {
                required_by: _,
                name,
                params,
                return_type: _,
                body: _,
            } => {
                let params_result: Result<Vec<_>, _> = params
                    .iter()
                    .map(|param| {
                        Ok(FuncParam {
                            label: param.label.clone(),
                            name: param.name.clone(),
                            value_type: None,
                            def_val: None,
                        })
                    })
                    .collect();

                type_def.funcs.push(Function {
                    name: name.to_string(),
                    params: params_result?,
                    return_type: None,
                    body: Vec::new(),
                    required_by: None,
                });
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
                let params_result: Result<Vec<_>, _> = params
                    .iter()
                    .map(|param| {
                        Ok(FuncParam {
                            label: param.label.clone(),
                            name: param.name.clone(),
                            value_type: None,
                            def_val: None,
                        })
                    })
                    .collect();

                type_def.inits.push(Initializer {
                    literal_bind: literal_bind.clone(),
                    params: params_result?,
                    body: Vec::new(),
                    required_by: None,
                });
            }

            _ => (),
        }
    }

    Ok(())
}
