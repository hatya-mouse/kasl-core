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
    ConstructorError, FuncParam, Function, Initializer, InputVar, Operator, OutputVar, ScopeVar,
    StateVar, SymbolPath, TypeDef,
};

pub trait Scope {
    /// Register the given Function to the scope.
    fn register_func(&mut self, _func: Function) -> Result<(), ConstructorError> {
        panic!("register_func not implemented on the Scope")
    }

    /// Get an immutable reference to a Function by its name. Returns None if the Function is not found.
    fn get_func(&self, _name: &str) -> Option<&Function> {
        None
    }

    /// Get a mutable reference to a Function by its name. Returns None if the Function is not found.
    fn get_func_mut(&mut self, _name: &str) -> Option<&mut Function> {
        None
    }

    /// Register the given Initializer to the scope.
    fn register_init(&mut self, _initializer: Initializer) -> Result<(), ConstructorError> {
        panic!("register_init not implemented on the Scope")
    }

    /// Get an immutable reference to an Initializer by its type of parameters. Returns None if the Initializer is not found.
    fn get_init(&self, _param_types: &[SymbolPath]) -> Option<&Initializer> {
        None
    }

    /// Get a mutable reference to an Initializer by its name. Returns None if the Initializer is not found.
    fn get_init_mut(&mut self, _param_types: &[SymbolPath]) -> Option<&mut Initializer> {
        None
    }

    /// Register the given TypeDef to the scope.
    fn register_type_def(&mut self, _type_def: TypeDef) -> Result<(), ConstructorError> {
        panic!("register_type_def not implemented on the Scope")
    }

    /// Get an immutable reference to a TypeDef by its name. Returns None if the TypeDef is not found.
    fn get_type_def(&self, _name: &str) -> Option<&TypeDef> {
        None
    }

    /// Get a mutable reference to a TypeDef by its name. Returns None if the TypeDef is not found.
    fn get_type_def_mut(&mut self, _name: &str) -> Option<&mut TypeDef> {
        None
    }

    /// Register the given InputVar to the scope.
    fn register_input(&mut self, _input: InputVar) -> Result<(), ConstructorError> {
        panic!("register_input not implemented on the Scope")
    }

    /// Get an immutable reference to an InputVar by its name. Returns None if the InputVar is not found.
    fn get_input(&self, _name: &str) -> Option<&InputVar> {
        None
    }

    /// Get a mutable reference to an InputVar by its name. Returns None if the InputVar is not found.
    fn get_input_mut(&mut self, _name: &str) -> Option<&mut InputVar> {
        None
    }

    /// Register the given OutputVar to the scope.
    fn register_output(&mut self, _output: OutputVar) -> Result<(), ConstructorError> {
        panic!("register_output not implemented on the Scope")
    }

    /// Get an immutable reference to an OutputVar by its name. Returns None if the OutputVar is not found.
    fn get_output(&self, _name: &str) -> Option<&OutputVar> {
        None
    }

    /// Get a mutable reference to an OutputVar by its name. Returns None if the OutputVar is not found.
    fn get_output_mut(&mut self, _name: &str) -> Option<&mut OutputVar> {
        None
    }

    /// Register the given StateVar to the scope.
    fn register_state(&mut self, _state: StateVar) -> Result<(), ConstructorError> {
        panic!("register_state not implemented on the Scope")
    }

    /// Get an immutable reference to a StateVar by its name. Returns None if the StateVar is not found.
    fn get_state(&self, _name: &str) -> Option<&StateVar> {
        None
    }

    /// Get a mutable reference to a StateVar by its name. Returns None if the StateVar is not found.
    fn get_state_mut(&mut self, _name: &str) -> Option<&mut StateVar> {
        None
    }

    /// Register the given ScopeVar to the scope.
    fn register_var(&mut self, _var: ScopeVar) -> Result<(), ConstructorError> {
        panic!("register_var not implemented on the Scope")
    }

    /// Get an immutable reference to a ScopeVar by its name. Returns None if the ScopeVar is not found.
    fn get_var(&self, _name: &str) -> Option<&ScopeVar> {
        None
    }

    /// Get a mutable reference to a ScopeVar by its name. Returns None if the ScopeVar is not found.
    fn get_var_mut(&mut self, _name: &str) -> Option<&mut ScopeVar> {
        None
    }

    /// Register the given Operator to the scope.
    fn register_operator(&mut self, _operator: Operator) -> Result<(), ConstructorError> {
        panic!("register_operator not implemented on the Scope")
    }

    /// Get an immutable reference to an Operator by its name. Returns None if the Operator is not found.
    fn get_operator(&self, _name: &str) -> Option<&Operator> {
        None
    }

    /// Get a mutable reference to an Operator by its name. Returns None if the Operator is not found.
    fn get_operator_mut(&mut self, _name: &str) -> Option<&mut Operator> {
        None
    }

    /// Register the given FuncParam to the scope.
    fn register_func_param(&mut self, _param: FuncParam) -> Result<(), ConstructorError> {
        panic!("register_func_param not implemented on the Scope")
    }

    /// Get an immutable reference to a FuncParam by its name. Returns None if the FuncParam is not found.
    fn get_func_param(&self, _name: &str) -> Option<&FuncParam> {
        None
    }

    /// Get a mutable reference to a FuncParam by its name. Returns None if the FuncParam is not found.
    fn get_func_param_mut(&mut self, _name: &str) -> Option<&mut FuncParam> {
        None
    }
}
