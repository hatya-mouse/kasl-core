//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::ast_nodes::NameSpaceID;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct NameSpace {
    pub id: NameSpaceID,
    child_namespaces: HashMap<String, NameSpaceID>,
}

impl NameSpace {
    pub fn new(id: NameSpaceID) -> Self {
        Self {
            id,
            child_namespaces: HashMap::new(),
        }
    }

    pub fn get_id_by_name(&self, name: &str) -> Option<NameSpaceID> {
        self.child_namespaces.get(name).copied()
    }

    pub fn add_child(&mut self, name: String, id: NameSpaceID) {
        self.child_namespaces.insert(name, id);
    }
}
