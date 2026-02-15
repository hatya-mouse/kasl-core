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

use crate::{ParserSymbolPath, Program, SymbolPath, SymbolPathComponent, SymbolTable};

impl Program {
    pub fn get_var_type(
        &self,
        parser_path: &ParserSymbolPath,
        symbol_table: &SymbolTable,
    ) -> Option<SymbolPath> {
        let symbol_path = symbol_table.resolve_path(parser_path)?;

        match symbol_path.components.last() {
            Some(last_component) => match last_component {
                SymbolPathComponent::InputVar(name) => {
                    // The path has been resolved so we can safely unwrap the input variable
                    Some(self.get_input(name).unwrap().value_type.clone())
                }
                SymbolPathComponent::OutputVar(name) => {
                    Some(self.get_output(name).unwrap().value_type.clone())
                }
                SymbolPathComponent::StateVar(name) => {
                    Some(self.get_state(name).unwrap().value_type.clone())
                }
                SymbolPathComponent::Var(_) => Some(
                    self.get_var_by_path(&symbol_path)
                        .unwrap()
                        .value_type
                        .clone(),
                ),
                _ => None,
            },
            None => None,
        }
    }

    /// Get the return type of a function.
    pub fn get_func_type(
        &self,
        parser_path: &ParserSymbolPath,
        symbol_table: &SymbolTable,
    ) -> Option<SymbolPath> {
        let func_symbol_path = match symbol_table.resolve_path(parser_path) {
            Some(path) => path,
            None => {
                return None;
            }
        };
        let func = match self.get_func_by_path(&func_symbol_path) {
            Some(func) => func,
            None => {
                return None;
            }
        };

        func.return_type.clone()
    }
}
