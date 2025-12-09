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
    ConstructorError, ConstructorErrorType, Function, Initializer, InputVar, Operator, OutputVar,
    Program, Range, ScopeVar, StateVar, SymbolPath, TypeDef,
};

impl Program {
    /// Register a Function to the program **by its path**.
    pub fn register_func_by_path(
        &mut self,
        func: Function,
        to_path: &SymbolPath,
    ) -> Result<(), ConstructorError> {
        let target_scope = match self.get_to_deepest_scope_mut(&to_path.components) {
            Some(scope) => scope,
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::TypeNotFound(to_path.clone()),
                    position: Range::zero(),
                });
            }
        };

        target_scope.register_func(func)?;

        Ok(())
    }

    /// Register a Initializer to the program **by its path**.
    pub fn register_init_by_path(
        &mut self,
        init: Initializer,
        to_path: &SymbolPath,
    ) -> Result<(), ConstructorError> {
        let target_scope = match self.get_to_deepest_scope_mut(&to_path.components) {
            Some(scope) => scope,
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::TypeNotFound(to_path.clone()),
                    position: Range::zero(),
                });
            }
        };

        target_scope.register_init(init)?;

        Ok(())
    }

    /// Register a TypeDef to the program **by its path**.
    pub fn register_type_def_by_path(
        &mut self,
        type_def: TypeDef,
        to_path: &SymbolPath,
    ) -> Result<(), ConstructorError> {
        let target_scope = match self.get_to_deepest_scope_mut(&to_path.components) {
            Some(scope) => scope,
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::TypeNotFound(to_path.clone()),
                    position: Range::zero(),
                });
            }
        };

        target_scope.register_type_def(type_def)?;

        Ok(())
    }

    /// Register an Operator to the program **by its path**.
    pub fn register_operator_by_path(
        &mut self,
        operator: Operator,
        to_path: &SymbolPath,
    ) -> Result<(), ConstructorError> {
        let target_scope = match self.get_to_deepest_scope_mut(&to_path.components) {
            Some(scope) => scope,
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::TypeNotFound(to_path.clone()),
                    position: Range::zero(),
                });
            }
        };

        target_scope.register_operator(operator)?;

        Ok(())
    }

    /// Register a ScopeVar to the program **by its path**.
    pub fn register_var_by_path(
        &mut self,
        var: ScopeVar,
        to_path: &SymbolPath,
    ) -> Result<(), ConstructorError> {
        let target_scope = match self.get_to_deepest_scope_mut(&to_path.components) {
            Some(scope) => scope,
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::TypeNotFound(to_path.clone()),
                    position: Range::zero(),
                });
            }
        };

        target_scope.register_var(var)?;

        Ok(())
    }

    /// Register an InputVar to the program **by its path**.
    pub fn register_input_by_path(
        &mut self,
        var: InputVar,
        to_path: &SymbolPath,
    ) -> Result<(), ConstructorError> {
        let target_scope = match self.get_to_deepest_scope_mut(&to_path.components) {
            Some(scope) => scope,
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::TypeNotFound(to_path.clone()),
                    position: Range::zero(),
                });
            }
        };

        target_scope.register_input(var)?;

        Ok(())
    }

    /// Register an OutputVar to the program **by its path**.
    pub fn register_output_by_path(
        &mut self,
        var: OutputVar,
        to_path: &SymbolPath,
    ) -> Result<(), ConstructorError> {
        let target_scope = match self.get_to_deepest_scope_mut(&to_path.components) {
            Some(scope) => scope,
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::TypeNotFound(to_path.clone()),
                    position: Range::zero(),
                });
            }
        };

        target_scope.register_output(var)?;

        Ok(())
    }

    /// Register a StateVar to the program **by its path**.
    pub fn register_state_by_path(
        &mut self,
        var: StateVar,
        to_path: &SymbolPath,
    ) -> Result<(), ConstructorError> {
        let target_scope = match self.get_to_deepest_scope_mut(&to_path.components) {
            Some(scope) => scope,
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::TypeNotFound(to_path.clone()),
                    position: Range::zero(),
                });
            }
        };

        target_scope.register_state(var)?;

        Ok(())
    }
}
