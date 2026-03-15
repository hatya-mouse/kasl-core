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
    NameSpace,
    scope_manager::{IOBlueprint, VariableKind},
};

pub struct BlueprintBuilder<'a> {
    namespace: &'a NameSpace,
}

impl<'a> BlueprintBuilder<'a> {
    pub fn new(namespace: &'a NameSpace) -> Self {
        Self { namespace }
    }

    pub fn build(&self) -> IOBlueprint {
        let mut blueprint = IOBlueprint::default();
        let global_scope = self.namespace.scope_registry.get_global_scope();

        // Loop over each variables in the global scope
        for var_id in &global_scope.variables {
            let Some(scope_var) = self.namespace.scope_registry.get_var_by_id(var_id) else {
                continue;
            };

            // If the variable is an input or output variable, add it to the blueprint
            match &scope_var.var_kind {
                VariableKind::Input { .. } => {
                    let input_size = self
                        .namespace
                        .type_registry
                        .get_type_size(&scope_var.value_type);
                    blueprint.add_input(input_size, scope_var.value_type, *var_id);
                }
                VariableKind::Output => {
                    let output_size = self
                        .namespace
                        .type_registry
                        .get_type_size(&scope_var.value_type);
                    blueprint.add_output(output_size, scope_var.value_type, *var_id);
                }
                VariableKind::State => {
                    let state_size = self
                        .namespace
                        .type_registry
                        .get_type_size(&scope_var.value_type);
                    blueprint.add_state(state_size, scope_var.value_type, *var_id);
                }
                _ => (),
            }
        }

        blueprint
    }
}
