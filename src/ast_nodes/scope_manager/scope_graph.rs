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

use crate::ast_nodes::ScopeID;
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
pub struct ScopeGraph {
    pub caller_to_callee: HashMap<ScopeID, HashSet<ScopeID>>,
}

impl ScopeGraph {
    pub fn add_edge(&mut self, caller: ScopeID, callee: ScopeID) {
        self.caller_to_callee
            .entry(caller)
            .or_default()
            .insert(callee);
    }

    pub fn get_callees(&self, caller: &ScopeID) -> Option<&HashSet<ScopeID>> {
        self.caller_to_callee.get(caller)
    }
}
