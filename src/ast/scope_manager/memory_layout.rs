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

use crate::VariableID;
use std::collections::HashMap;

pub struct MemoryLayout {
    /// Total size of the memory layout in bytes.
    pub total_size: usize,
    /// Offset of each variables in bytes.
    pub offsets: HashMap<VariableID, usize>,
}

impl MemoryLayout {
    pub fn new() -> Self {
        Self {
            total_size: 0,
            offsets: HashMap::new(),
        }
    }

    pub fn register_offset(&mut self, var_id: VariableID, offset: usize) {
        self.offsets.insert(var_id, offset);
    }

    pub fn set_total_size(&mut self, size: usize) {
        self.total_size = size;
    }
}
