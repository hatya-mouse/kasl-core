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

use crate::{ConstructorError, ParserStatementKind, Program, ScopeVar, SymbolPath, SymbolTable};

pub fn collect_member_variables(
    program: &mut Program,
    symbol_table: &SymbolTable,
    scope_path: &SymbolPath,
) -> Result<(), ConstructorError> {
    for stmt in &symbol_table.vars {
        match &stmt.1.kind {
            ParserStatementKind::Var {
                required_by: _,
                name,
                value_type: _,
                def_val: _,
            } => {
                let var = ScopeVar {
                    required_by: None,
                    name: name.clone(),
                    value_type: None,
                    def_val: None,
                };
                program.register_var_by_path(var, scope_path)?;
            }

            _ => (),
        }
    }

    Ok(())
}
