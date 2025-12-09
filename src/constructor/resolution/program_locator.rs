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
    FuncParam, InputVar, OutputVar, Program, ScopeVar, StateVar, SymbolPath, SymbolPathComponent,
};

pub trait ProgramLocator {
    /// Get mutable reference to inferable variable
    fn get_inferable_var_mut(&mut self, symbol_path: &SymbolPath) -> Option<&mut ScopeVar>;

    /// Get mutable reference to inferable input variable
    fn get_inferable_input_mut(&mut self, symbol_path: &SymbolPath) -> Option<&mut InputVar>;

    /// Get mutable reference to inferable output variable
    fn get_inferable_output_mut(&mut self, symbol_path: &SymbolPath) -> Option<&mut OutputVar>;

    /// Get mutable reference to inferable state variable
    fn get_inferable_state_mut(&mut self, symbol_path: &SymbolPath) -> Option<&mut StateVar>;

    /// Get mutable reference to inferable function parameter variable
    fn get_inferable_func_param_mut(&mut self, symbol_path: &SymbolPath) -> Option<&mut FuncParam>;
}

impl ProgramLocator for Program {
    fn get_inferable_var_mut(&mut self, symbol_path: &SymbolPath) -> Option<&mut ScopeVar> {
        if symbol_path.components.is_empty() {
            return None;
        }
        let (last, parent) = symbol_path.components.split_last()?;

        let parent_scope = self.get_to_deepest_scope_mut(parent)?;

        match last {
            SymbolPathComponent::Var(name) => parent_scope.get_var_mut(name),
            _ => None,
        }
    }

    fn get_inferable_input_mut(&mut self, symbol_path: &SymbolPath) -> Option<&mut InputVar> {
        if symbol_path.components.is_empty() {
            return None;
        }
        let (last, parent) = symbol_path.components.split_last()?;

        let parent_scope = self.get_to_deepest_scope_mut(parent)?;

        match last {
            SymbolPathComponent::InputVar(name) => parent_scope.get_input_mut(name),
            _ => None,
        }
    }

    fn get_inferable_output_mut(&mut self, symbol_path: &SymbolPath) -> Option<&mut OutputVar> {
        if symbol_path.components.is_empty() {
            return None;
        }
        let (last, parent) = symbol_path.components.split_last()?;

        let parent_scope = self.get_to_deepest_scope_mut(parent)?;

        match last {
            SymbolPathComponent::OutputVar(name) => parent_scope.get_output_mut(name),
            _ => None,
        }
    }

    fn get_inferable_state_mut(&mut self, symbol_path: &SymbolPath) -> Option<&mut StateVar> {
        if symbol_path.components.is_empty() {
            return None;
        }
        let (last, parent) = symbol_path.components.split_last()?;

        let parent_scope = self.get_to_deepest_scope_mut(parent)?;

        match last {
            SymbolPathComponent::StateVar(name) => parent_scope.get_state_mut(name),
            _ => None,
        }
    }

    fn get_inferable_func_param_mut<'a>(
        &'a mut self,
        symbol_path: &SymbolPath,
    ) -> Option<&'a mut FuncParam> {
        if symbol_path.components.is_empty() {
            return None;
        }
        let (last, parent) = symbol_path.components.split_last()?;

        let parent_scope = self.get_to_deepest_scope_mut(parent)?;

        match last {
            SymbolPathComponent::FuncParam(name) => parent_scope.get_func_param_mut(name),
            _ => None,
        }
    }
}
