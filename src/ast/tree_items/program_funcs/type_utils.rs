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
    ParserSymbolPath, ParserSymbolPathComponent, Program, ScopeItemMut, SymbolPath,
    SymbolPathComponent, tree_items::ScopeItem,
};

impl Program {
    /// Get an **immutable** reference to the scope which belongs to the last component of the given vector of the symbol path component.
    pub fn get_to_deepest_scope<'a>(
        &'a self,
        path_components: &[SymbolPathComponent],
    ) -> Option<ScopeItem<'a>> {
        Self::resolve_path_recursive(ScopeItem::Program(self), path_components)
    }

    /// Get a **mutable** reference to the scope which belongs to the last component of the given vector of the symbol path component.
    pub fn get_to_deepest_scope_mut<'a>(
        &'a mut self,
        path_components: &[SymbolPathComponent],
    ) -> Option<ScopeItemMut<'a>> {
        Self::resolve_path_recursive_mut(ScopeItemMut::Program(self), path_components)
    }

    /// Resolve ParserSymbolPath to obtain a SymbolPath to the TypeDef.
    pub fn resolve_type_def_parser_path(
        &self,
        parser_symbol_path: &ParserSymbolPath,
    ) -> Option<SymbolPath> {
        Self::resolve_type_def_recursive(
            ScopeItem::Program(self),
            &parser_symbol_path.path,
            SymbolPath::new(),
        )
    }

    // -- Private Recursive Helper Functions --
    fn resolve_path_recursive<'a>(
        current_scope: ScopeItem<'a>,
        path: &[SymbolPathComponent],
    ) -> Option<ScopeItem<'a>> {
        if path.is_empty() {
            return Some(current_scope);
        }

        let (head, rest) = path.split_first().unwrap();
        let next_scope_item = match (current_scope, head) {
            (ScopeItem::Program(prog), SymbolPathComponent::TypeDef(name)) => {
                prog.get_type_def(name).map(ScopeItem::TypeDef)
            }
            (ScopeItem::TypeDef(td), SymbolPathComponent::TypeDef(name)) => {
                td.get_type_def(name).map(ScopeItem::TypeDef)
            }
            (ScopeItem::Program(prog), SymbolPathComponent::Func(name)) => {
                prog.get_func(name).map(ScopeItem::Func)
            }
            (ScopeItem::TypeDef(td), SymbolPathComponent::Func(name)) => {
                td.get_func(name).map(ScopeItem::Func)
            }
            _ => None,
        }?;

        Self::resolve_path_recursive(next_scope_item, rest)
    }

    fn resolve_path_recursive_mut<'a>(
        current_scope: ScopeItemMut<'a>,
        path: &[SymbolPathComponent],
    ) -> Option<ScopeItemMut<'a>> {
        if path.is_empty() {
            return Some(current_scope);
        }

        let (head, rest) = path.split_first().unwrap();
        let next_scope_item = match (current_scope, head) {
            (ScopeItemMut::Program(prog), SymbolPathComponent::TypeDef(name)) => {
                prog.get_type_def_mut(name).map(ScopeItemMut::TypeDef)
            }
            (ScopeItemMut::TypeDef(td), SymbolPathComponent::TypeDef(name)) => {
                td.get_type_def_mut(name).map(ScopeItemMut::TypeDef)
            }
            (ScopeItemMut::Program(prog), SymbolPathComponent::Func(name)) => {
                prog.get_func_mut(name).map(ScopeItemMut::Func)
            }
            (ScopeItemMut::TypeDef(td), SymbolPathComponent::Func(name)) => {
                td.get_func_mut(name).map(ScopeItemMut::Func)
            }
            _ => None,
        }?;

        Self::resolve_path_recursive_mut(next_scope_item, rest)
    }

    fn resolve_type_def_recursive<'a>(
        current_scope: ScopeItem<'a>,
        parser_path_rest: &[ParserSymbolPathComponent],
        mut accumulated_path: SymbolPath,
    ) -> Option<SymbolPath> {
        if parser_path_rest.is_empty() {
            return Some(accumulated_path);
        }

        let (head, tail) = parser_path_rest.split_first().unwrap();

        let (next_scope_item, component_to_add) = match (current_scope, &head.symbol) {
            (ScopeItem::Program(prog), symbol_name) => {
                let next_type_def = prog.get_type_def(symbol_name)?;
                (
                    ScopeItem::TypeDef(next_type_def),
                    SymbolPathComponent::TypeDef(symbol_name.clone()),
                )
            }
            (ScopeItem::TypeDef(td), symbol_name) => {
                let next_type_def = td.get_type_def(symbol_name)?;
                (
                    ScopeItem::TypeDef(next_type_def),
                    SymbolPathComponent::TypeDef(symbol_name.clone()),
                )
            }
            _ => return None,
        };

        accumulated_path.push(component_to_add);

        Self::resolve_type_def_recursive(next_scope_item, tail, accumulated_path)
    }
}
