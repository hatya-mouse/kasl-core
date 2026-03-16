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

mod field_access_resolver;
mod identifier_resolver;
mod namespace_resolver;
mod recursive_resolver;

use crate::{
    ScopeID,
    error::ErrorCollector,
    namespace_registry::{NameSpacePair, NameSpaceRegistry},
};

pub struct LValueResolver<'a> {
    ec: &'a mut ErrorCollector,
    namespace_registry: &'a NameSpaceRegistry,
    current_scope: NameSpacePair<ScopeID>,
}

impl<'a> LValueResolver<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        namespace_registry: &'a NameSpaceRegistry,
        current_scope: NameSpacePair<ScopeID>,
    ) -> Self {
        Self {
            ec,
            namespace_registry,
            current_scope,
        }
    }
}
