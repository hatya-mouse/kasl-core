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

use crate::type_registry::ResolvedType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct ArrayDecl {
    item_type: ResolvedType,
    count: u32,
}

impl ArrayDecl {
    pub fn new(item_type: ResolvedType, count: u32) -> Self {
        Self { item_type, count }
    }

    pub fn item_type(&self) -> &ResolvedType {
        &self.item_type
    }

    pub fn count(&self) -> &u32 {
        &self.count
    }
}
