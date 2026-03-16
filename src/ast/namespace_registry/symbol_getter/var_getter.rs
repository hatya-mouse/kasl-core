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
    ScopeID, ScopeVar, VariableID,
    namespace_registry::{NameSpacePair, NameSpaceRegistry},
};

pub trait NameSpaceVarGetter {
    fn get_var_id(
        &self,
        scope_id: &NameSpacePair<ScopeID>,
        var_name: &str,
    ) -> Option<NameSpacePair<VariableID>>;

    fn get_var(&self, id: &NameSpacePair<VariableID>) -> Option<&ScopeVar>;
}

impl NameSpaceVarGetter for NameSpaceRegistry {
    // --- TYPE RESOLUTION ---

    fn get_var_id(
        &self,
        scope_id: &NameSpacePair<ScopeID>,
        var_name: &str,
    ) -> Option<NameSpacePair<VariableID>> {
        let namespace = self.get_namespace_by_id(&scope_id.namespace_id)?;
        namespace
            .scope_registry
            .lookup_var(scope_id.symbol_id, var_name)
            .map(|var_id| NameSpacePair {
                namespace_id: scope_id.namespace_id,
                symbol_id: *var_id,
            })
    }

    fn get_var(&self, id: &NameSpacePair<VariableID>) -> Option<&ScopeVar> {
        let namespace = self.get_namespace_by_id(&id.namespace_id)?;
        namespace.scope_registry.get_var_by_id(&id.symbol_id)
    }
}
