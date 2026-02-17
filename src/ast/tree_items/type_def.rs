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

use crate::{Function, InfixOperator, Initializer, PrefixOperator, ScopeVar, SymbolPath};

#[derive(Debug, PartialEq, Clone)]
pub struct TypeDef {
    pub name: String,
    pub inherits: Vec<SymbolPath>,
    pub vars: Vec<ScopeVar>,
    pub inits: Vec<Initializer>,
    pub funcs: Vec<Function>,
    pub types: Vec<TypeDef>,
    pub infix_operators: Vec<InfixOperator>,
    pub prefix_operators: Vec<PrefixOperator>,
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
            infix_operators: Vec::new(),
            prefix_operators: Vec::new(),
        }
    }

    // -- TypeDef --
    pub fn get_type_def(&self, name: &str) -> Option<&TypeDef> {
        self.types.iter().find(|s| s.name == name)
    }

    pub fn get_type_def_mut(&mut self, name: &str) -> Option<&mut TypeDef> {
        self.types.iter_mut().find(|s| s.name == name)
    }

    // -- Function --
    pub fn register_func(&mut self, func: Function) {
        self.funcs.push(func);
    }

    pub fn get_func(&self, name: &str) -> Option<&Function> {
        self.funcs.iter().find(|f| f.name == name)
    }

    pub fn get_func_mut(&mut self, name: &str) -> Option<&mut Function> {
        self.funcs.iter_mut().find(|f| f.name == name)
    }

    // -- Initializer --
    pub fn register_init(&mut self, init: Initializer) {
        self.inits.push(init);
    }

    // -- ScopeVar --
    pub fn register_var(&mut self, var: ScopeVar) {
        self.vars.push(var);
    }

    pub fn get_var(&self, name: &str) -> Option<&ScopeVar> {
        self.vars.iter().find(|v| v.name == name)
    }

    pub fn get_var_mut(&mut self, name: &str) -> Option<&mut ScopeVar> {
        self.vars.iter_mut().find(|v| v.name == name)
    }
}
