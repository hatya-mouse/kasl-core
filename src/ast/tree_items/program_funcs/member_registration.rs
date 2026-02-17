//
// © 2025-2026 Shuntaro Kasatani
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
    Function, Initializer, Program, Range, ScopeItemMut, ScopeVar, SymbolPath,
    error::{ErrorCollector, Phase},
};

impl Program {
    /// Register a Function to the program **by its path**.
    /// The path should not include the function.
    pub fn register_func_by_path(
        &mut self,
        ec: &mut ErrorCollector,
        func: Function,
        to_path: &SymbolPath,
        decl_range: Range,
    ) {
        let target_scope = match self.get_to_deepest_scope_mut(&to_path.components) {
            Some(scope) => scope,
            None => {
                ec.comp_bug(
                    decl_range,
                    Phase::TypeResolution,
                    &format!("Could not reach the deepest scope for path {:?}", to_path),
                );
                return;
            }
        };

        match target_scope {
            ScopeItemMut::Program(prog) => prog.register_func(func),
            ScopeItemMut::TypeDef(td) => td.register_func(func),
            other => {
                ec.comp_bug(
                    decl_range,
                    Phase::TypeResolution,
                    &format!(
                        "Expected TypeDef or Program scope for path {:?}, found {:?}",
                        to_path, other
                    ),
                );
            }
        }
    }

    /// Register a Initializer to the program **by its path**.
    pub fn register_init_by_path(
        &mut self,
        ec: &mut ErrorCollector,
        init: Initializer,
        to_path: &SymbolPath,
        decl_range: Range,
    ) {
        let target_scope = match self.get_to_deepest_scope_mut(&to_path.components) {
            Some(scope) => scope,
            None => {
                ec.comp_bug(
                    decl_range,
                    Phase::TypeResolution,
                    &format!("Could not reach the deepest scope for path {:?}", to_path),
                );
                return;
            }
        };

        match target_scope {
            ScopeItemMut::TypeDef(td) => td.register_init(init),
            other => {
                ec.comp_bug(
                    decl_range,
                    Phase::TypeResolution,
                    &format!(
                        "Expected TypeDef scope for path {:?}, found {:?}",
                        to_path, other
                    ),
                );
            }
        }
    }

    /// Register a ScopeVar to the program **by its path**.
    pub fn register_var_by_path(
        &mut self,
        ec: &mut ErrorCollector,
        var: ScopeVar,
        to_path: &SymbolPath,
        decl_range: Range,
    ) {
        let target_scope = match self.get_to_deepest_scope_mut(&to_path.components) {
            Some(scope) => scope,
            None => {
                ec.comp_bug(
                    decl_range,
                    Phase::TypeResolution,
                    &format!("Could not reach the deepest scope for path {:?}", to_path),
                );
                return;
            }
        };

        match target_scope {
            ScopeItemMut::TypeDef(td) => td.register_var(var),
            other => {
                ec.comp_bug(
                    decl_range,
                    Phase::TypeResolution,
                    &format!(
                        "Expected TypeDef scope for path {:?}, found {:?}",
                        to_path, other
                    ),
                );
            }
        }
    }
}
