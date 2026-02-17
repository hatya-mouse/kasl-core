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
    FuncParam, Function, Program, ScopeItem, ScopeItemMut, ScopeVar, SymbolPath,
    SymbolPathComponent,
};

impl Program {
    /// Get an immutable reference to the Function by its path.
    pub fn get_func_by_path(&self, symbol_path: &SymbolPath) -> Option<&Function> {
        let target_scope = self.get_to_deepest_scope(&symbol_path.components)?;
        match target_scope {
            ScopeItem::Func(func) => Some(func),
            _ => None,
        }
    }

    /// Get an immutable reference to the ScopeVar by its path.
    pub fn get_var_by_path(&self, symbol_path: &SymbolPath) -> Option<&ScopeVar> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope(parent)?;

        match last {
            SymbolPathComponent::Var(name) => match parent_scope {
                ScopeItem::TypeDef(td) => td.get_var(name),
                _ => None,
            },
            _ => None,
        }
    }

    /// Get a mutable reference to the ScopeVar by its path.
    pub fn get_var_by_path_mut(&mut self, symbol_path: &SymbolPath) -> Option<&mut ScopeVar> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope_mut(parent)?;

        match last {
            SymbolPathComponent::Var(name) => match parent_scope {
                ScopeItemMut::TypeDef(td) => td.get_var_mut(name),
                _ => None,
            },
            _ => None,
        }
    }
}
