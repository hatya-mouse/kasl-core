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

use crate::ast_nodes::StructID;
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
pub struct StructGraph {
    pub parent_to_field: HashMap<StructID, HashSet<StructID>>,
}

impl StructGraph {
    pub fn add_edge(&mut self, parent: StructID, field: StructID) {
        self.parent_to_field
            .entry(parent)
            .or_default()
            .insert(field);
    }

    pub fn get_fields(&self, parent: &StructID) -> Option<&HashSet<StructID>> {
        self.parent_to_field.get(parent)
    }

    pub fn get_root_nodes(&self) -> HashSet<StructID> {
        let used_structs = self
            .parent_to_field
            .values()
            .flatten()
            .collect::<HashSet<_>>();
        self.parent_to_field
            .keys()
            .filter(|node| !used_structs.contains(node))
            .copied()
            .collect()
    }
}
