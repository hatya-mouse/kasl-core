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

use crate::{ConstructorError, Function, Initializer, Operator, Scope, ScopeVar, SymbolPath};

#[derive(Debug, PartialEq, Clone)]
pub struct TypeDef {
    pub name: String,
    pub inherits: Vec<SymbolPath>,
    pub vars: Vec<ScopeVar>,
    pub inits: Vec<Initializer>,
    pub funcs: Vec<Function>,
    pub types: Vec<TypeDef>,
    pub operators: Vec<Operator>,
}

impl TypeDef {
    pub fn new(name: String) -> Self {
        TypeDef {
            name,
            inherits: Vec::new(),
            vars: Vec::new(),
            inits: Vec::new(),
            funcs: Vec::new(),
            types: Vec::new(),
            operators: Vec::new(),
        }
    }

    pub fn get_type_def_mut(&mut self, name: &str) -> Option<&mut TypeDef> {
        self.types.iter_mut().find(|s| s.name == name)
    }
}

impl Scope for TypeDef {
    fn register_func(&mut self, func: Function) -> Result<(), ConstructorError> {
        self.funcs.push(func);
        Ok(())
    }

    fn get_func(&self, name: &str) -> Option<&Function> {
        self.funcs.iter().find(|f| f.name == name)
    }

    fn get_func_mut(&mut self, name: &str) -> Option<&mut Function> {
        self.funcs.iter_mut().find(|f| f.name == name)
    }

    fn register_initializer(&mut self, initializer: Initializer) -> Result<(), ConstructorError> {
        self.inits.push(initializer);
        Ok(())
    }

    fn get_initializer(&self, param_types: &[SymbolPath]) -> Option<&Initializer> {
        self.inits.iter().find(|i| i.does_params_match(param_types))
    }

    fn get_initializer_mut(&mut self, param_types: &[SymbolPath]) -> Option<&mut Initializer> {
        self.inits
            .iter_mut()
            .find(|i| i.does_params_match(param_types))
    }

    fn register_type_def(&mut self, type_def: TypeDef) -> Result<(), ConstructorError> {
        self.types.push(type_def);
        Ok(())
    }

    fn get_type_def(&self, name: &str) -> Option<&TypeDef> {
        self.types.iter().find(|t| t.name == name)
    }

    fn get_type_def_mut(&mut self, name: &str) -> Option<&mut TypeDef> {
        self.types.iter_mut().find(|t| t.name == name)
    }

    fn register_var(&mut self, var: ScopeVar) -> Result<(), ConstructorError> {
        self.vars.push(var);
        Ok(())
    }

    fn get_var(&self, name: &str) -> Option<&ScopeVar> {
        self.vars.iter().find(|v| v.name == name)
    }

    fn get_var_mut(&mut self, name: &str) -> Option<&mut ScopeVar> {
        self.vars.iter_mut().find(|v| v.name == name)
    }

    fn register_operator(&mut self, operator: Operator) -> Result<(), ConstructorError> {
        self.operators.push(operator);
        Ok(())
    }

    fn get_operator(&self, name: &str) -> Option<&Operator> {
        self.operators.iter().find(|o| o.symbol == name)
    }

    fn get_operator_mut(&mut self, name: &str) -> Option<&mut Operator> {
        self.operators.iter_mut().find(|o| o.symbol == name)
    }
}
