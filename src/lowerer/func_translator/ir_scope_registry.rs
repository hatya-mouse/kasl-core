//
// Copyright 2025-2026 Shuntaro Kasatani
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

use crate::ast::VariableID;
use kasl_ir::Variable;
use std::collections::HashMap;

#[derive(Default)]
pub struct IRScope {
    variables: HashMap<VariableID, Variable>,
}

impl IRScope {
    pub fn get_var(&self, var_id: &VariableID) -> Option<Variable> {
        self.variables.get(var_id).copied()
    }

    pub fn or_insert_var(&mut self, var_id: VariableID, var: Variable) -> Variable {
        *self.variables.entry(var_id).or_insert(var)
    }
}

#[derive(Default)]
pub struct IRScopeRegistry {
    current_scopes: Vec<IRScope>,
}

impl IRScopeRegistry {
    pub fn get_var(&self, var_id: &VariableID) -> Variable {
        self.current_scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get_var(var_id))
            .unwrap()
    }

    pub fn add_var(&mut self, var_id: VariableID, var: Variable) -> Variable {
        self.current_scopes
            .last_mut()
            .unwrap()
            .or_insert_var(var_id, var)
    }

    pub fn pop_deepest(&mut self) {
        self.current_scopes.pop();
    }

    pub fn push_deepest(&mut self) {
        self.current_scopes.push(IRScope::default());
    }
}
