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

use crate::{ParserSymbolPath, Program, Scope, SymbolPath, SymbolPathComponent, TypeDef};

impl Program {
    /// Resolve ParserSymbolPath to obtain a SymbolPath to the TypeDef.
    pub fn resolve_type_def_parser_path(
        &self,
        parser_symbol_path: &ParserSymbolPath,
    ) -> Option<SymbolPath> {
        let mut current_scope: &dyn Scope = self;
        let mut complete_path = SymbolPath::new();

        // Loop through each component in the ParserSymbolPath
        for component in parser_symbol_path {
            // Check if the current scope contains a TypeDef with the given symbol
            match current_scope.get_type_def(&component.symbol) {
                Some(next_scope) => {
                    // If it does, push the TypeDef component to the complete path and update the current scope
                    let type_def_name = component.symbol.clone();
                    let new_component = SymbolPathComponent::TypeDef(type_def_name);
                    complete_path.push(new_component);
                    current_scope = next_scope;
                }
                _ => return None,
            }
        }

        Some(complete_path)
    }

    /// Get a immutable reference to the TypeDef by its path.
    pub fn get_type_def_by_path(&self, symbol_path: &SymbolPath) -> Option<&TypeDef> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope(parent)?;

        match last {
            SymbolPathComponent::TypeDef(name) => parent_scope.get_type_def(name),
            _ => None,
        }
    }

    /// Get a mutable reference to the TypeDef by its path.
    pub fn get_type_def_by_path_mut(&mut self, symbol_path: &SymbolPath) -> Option<&mut TypeDef> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope_mut(parent)?;

        match last {
            SymbolPathComponent::TypeDef(name) => parent_scope.get_type_def_mut(name),
            _ => None,
        }
    }

    /// Get an **immutable** reference to the scope which belongs to the last component of the given vector of the symbol path component.
    pub fn get_to_deepest_scope(
        &self,
        path_components: &[SymbolPathComponent],
    ) -> Option<&dyn Scope> {
        let mut current_scope: &dyn Scope = self;

        for comp in path_components {
            match comp {
                SymbolPathComponent::TypeDef(name) => {
                    current_scope = current_scope.get_type_def(name)?;
                }
                _ => return None,
            }
        }

        Some(current_scope)
    }

    /// Get a **mutable** reference to the scope which belongs to the last component of the given vector of the symbol path component.
    pub fn get_to_deepest_scope_mut(
        &mut self,
        path_components: &[SymbolPathComponent],
    ) -> Option<&mut dyn Scope> {
        let mut current_scope: &mut dyn Scope = self;

        for comp in path_components {
            match comp {
                SymbolPathComponent::TypeDef(name) => {
                    current_scope = current_scope.get_type_def_mut(name)?;
                }
                _ => return None,
            }
        }

        Some(current_scope)
    }
}
