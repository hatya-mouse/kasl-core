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
    VariableID,
    scope_manager::{ScopeID, scope_var::ScopeVar},
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scope {
    pub parent: Option<ScopeID>,
    name_to_id: HashMap<String, VariableID>,
}

impl Scope {
    pub fn new(parent: Option<ScopeID>) -> Self {
        Self {
            parent,
            name_to_id: HashMap::new(),
        }
    }

    pub fn get_id_by_name(&self, name: &str) -> Option<&VariableID> {
        self.name_to_id.get(name)
    }

    pub fn register_var(&mut self, name: String, id: VariableID) {
        self.name_to_id.insert(name, id);
    }
}
