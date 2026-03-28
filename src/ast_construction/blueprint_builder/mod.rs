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

use crate::ast::{
    NameSpaceID,
    compilation_data::ProgramContext,
    scope_manager::{BlueprintItem, IOBlueprint, VariableKind},
};

pub struct BlueprintBuilder<'a> {
    prog_ctx: &'a ProgramContext,
}

impl<'a> BlueprintBuilder<'a> {
    pub fn new(prog_ctx: &'a ProgramContext) -> Self {
        Self { prog_ctx }
    }

    pub fn build(&self) -> IOBlueprint {
        let mut blueprint = IOBlueprint::default();
        let namespaces = self.prog_ctx.namespace_registry.get_all_namespace_ids();
        for namespace_id in namespaces {
            self.build_namespace(&mut blueprint, &namespace_id);
        }
        blueprint
    }

    fn build_namespace(&self, blueprint: &mut IOBlueprint, namespace_id: &NameSpaceID) {
        let global_scope = self.prog_ctx.scope_registry.get_global_scope(namespace_id);

        // Loop over each variables in the global scope
        for var_id in &global_scope.variables {
            let Some(scope_var) = self.prog_ctx.scope_registry.get_var(var_id) else {
                continue;
            };

            // If the variable is an input or output variable, add it to the blueprint
            let actual_size = self
                .prog_ctx
                .type_registry
                .get_type_actual_size(&scope_var.value_type)
                .unwrap();
            let align = self
                .prog_ctx
                .type_registry
                .get_type_alignment(&scope_var.value_type)
                .unwrap();
            // Input/Output/State variables must have a default value
            let item = BlueprintItem {
                name: scope_var.name.clone(),
                actual_size,
                align,
                value_type: scope_var.value_type,
                def_val: scope_var.def_val.clone().unwrap(),
                id: *var_id,
            };

            match &scope_var.var_kind {
                VariableKind::Input { .. } => {
                    blueprint.add_input(item);
                }
                VariableKind::Output => {
                    blueprint.add_output(item);
                }
                VariableKind::State => {
                    blueprint.add_state(item);
                }
                _ => (),
            }
        }
    }
}
