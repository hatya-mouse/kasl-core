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
    FuncParam, Function, Initializer, InputVar, OutputVar, Program, ScopeVar, StateVar, SymbolPath,
    SymbolPathComponent, TypeDef,
};

impl Program {
    /// Get an immutable reference to the Function by its path.
    pub fn get_func_by_path(&self, symbol_path: &SymbolPath) -> Option<&Function> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope(parent)?;

        match last {
            SymbolPathComponent::Func(name) => parent_scope.get_func(name),
            _ => None,
        }
    }

    /// Get a mutable reference to the Function by its path.
    pub fn get_func_by_path_mut(&mut self, symbol_path: &SymbolPath) -> Option<&mut Function> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope_mut(parent)?;

        match last {
            SymbolPathComponent::Func(name) => parent_scope.get_func_mut(name),
            _ => None,
        }
    }

    /// Get an immutable reference to the Initializer by its path and parameter types.
    pub fn get_init_by_path(
        &self,
        type_path: &SymbolPath,
        param_types: &[SymbolPath],
    ) -> Option<&Initializer> {
        let parent_scope = self.get_to_deepest_scope(&type_path.components)?;
        parent_scope.get_init(param_types)
    }

    /// Get a mutable reference to the Initializer by its path and parameter types.
    pub fn get_init_by_path_mut(
        &mut self,
        type_path: &SymbolPath,
        param_types: &[SymbolPath],
    ) -> Option<&mut Initializer> {
        let parent_scope = self.get_to_deepest_scope_mut(&type_path.components)?;
        parent_scope.get_init_mut(param_types)
    }

    /// Get an immutable reference to the TypeDef by its path.
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

    /// Get an immutable reference to the InputVar by its path.
    pub fn get_input_by_path(&self, symbol_path: &SymbolPath) -> Option<&InputVar> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope(parent)?;

        match last {
            SymbolPathComponent::InputVar(name) => parent_scope.get_input(name),
            _ => None,
        }
    }

    /// Get a mutable reference to the InputVar by its path.
    pub fn get_input_by_path_mut(&mut self, symbol_path: &SymbolPath) -> Option<&mut InputVar> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope_mut(parent)?;

        match last {
            SymbolPathComponent::InputVar(name) => parent_scope.get_input_mut(name),
            _ => None,
        }
    }

    /// Get an immutable reference to the OutputVar by its path.
    pub fn get_output_by_path(&self, symbol_path: &SymbolPath) -> Option<&OutputVar> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope(parent)?;

        match last {
            SymbolPathComponent::OutputVar(name) => parent_scope.get_output(name),
            _ => None,
        }
    }

    /// Get a mutable reference to the OutputVar by its path.
    pub fn get_output_by_path_mut(&mut self, symbol_path: &SymbolPath) -> Option<&mut OutputVar> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope_mut(parent)?;

        match last {
            SymbolPathComponent::OutputVar(name) => parent_scope.get_output_mut(name),
            _ => None,
        }
    }

    /// Get an immutable reference to the StateVar by its path.
    pub fn get_state_by_path(&self, symbol_path: &SymbolPath) -> Option<&StateVar> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope(parent)?;

        match last {
            SymbolPathComponent::StateVar(name) => parent_scope.get_state(name),
            _ => None,
        }
    }

    /// Get a mutable reference to the StateVar by its path.
    pub fn get_state_by_path_mut(&mut self, symbol_path: &SymbolPath) -> Option<&mut StateVar> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope_mut(parent)?;

        match last {
            SymbolPathComponent::StateVar(name) => parent_scope.get_state_mut(name),
            _ => None,
        }
    }

    /// Get an immutable reference to the ScopeVar by its path.
    pub fn get_var_by_path(&self, symbol_path: &SymbolPath) -> Option<&ScopeVar> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope(parent)?;

        match last {
            SymbolPathComponent::Var(name) => parent_scope.get_var(name),
            _ => None,
        }
    }

    /// Get a mutable reference to the ScopeVar by its path.
    pub fn get_var_by_path_mut(&mut self, symbol_path: &SymbolPath) -> Option<&mut ScopeVar> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope_mut(parent)?;

        match last {
            SymbolPathComponent::Var(name) => parent_scope.get_var_mut(name),
            _ => None,
        }
    }

    /// Get an immutable reference to the FuncParam by its path.
    pub fn get_func_param_by_path(&self, symbol_path: &SymbolPath) -> Option<&FuncParam> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope(parent)?;

        match last {
            SymbolPathComponent::FuncParam(name) => parent_scope.get_func_param(name),
            _ => None,
        }
    }

    /// Get a mutable reference to the FuncParam by its path.
    pub fn get_func_param_by_path_mut(
        &mut self,
        symbol_path: &SymbolPath,
    ) -> Option<&mut FuncParam> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope_mut(parent)?;

        match last {
            SymbolPathComponent::FuncParam(name) => parent_scope.get_func_param_mut(name),
            _ => None,
        }
    }
}
