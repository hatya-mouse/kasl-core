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
    NameSpaceID, OperatorContext, ScopeRegistry, namespace_registry::NameSpaceRegistry,
    symbol_table::FunctionContext, type_registry::TypeRegistry,
};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct NameSpace {
    pub namespace_id: NameSpaceID,
    pub name_to_id: HashMap<String, NameSpaceID>,

    pub func_ctx: FunctionContext,
    pub op_ctx: OperatorContext,
    pub scope_registry: ScopeRegistry,
    pub type_registry: TypeRegistry,
}

impl NameSpace {
    pub fn get_id_by_name(&self, name: &str) -> Option<NameSpaceID> {
        self.name_to_id.get(name).copied()
    }
}
