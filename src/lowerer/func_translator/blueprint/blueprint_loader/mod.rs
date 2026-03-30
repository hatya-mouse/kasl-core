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

mod item_loader;

use crate::{
    ast::{
        VariableID,
        scope_manager::{BlueprintItem, IOBlueprint, VariableKind},
        type_registry::ResolvedType,
    },
    lowerer::{TranslatorParams, func_translator::FuncTranslator},
};
use kasl_ir::ir::{Const, InstBuilder, IntBinOp, IntCmp, Offset, Value};

impl FuncTranslator<'_> {
    pub fn load_blueprint_access(
        &mut self,
        params: &TranslatorParams,
        blueprint: &IOBlueprint,
        iteration: Option<Value>,
    ) {
        // Assume that the inputs, outputs and states are packed in the order they are declared
        let mut input_count: u32 = 0;
        let mut state_count: u32 = 0;

        // Calculate whether the states should be initialized
        let i32_zero = self.builder.const_val(Const::I32(0));
        let is_first = iteration
            .map(|index| self.builder.icmp(IntCmp::Eq, index, i32_zero))
            .unwrap_or_else(|| self.builder.const_val(Const::I8(1)));
        let is_first_and_should_init =
            self.builder
                .ibop(IntBinOp::BAnd, is_first, params.should_init);

        // Get the all namespaces
        let namespaces = self.prog_ctx.namespace_registry.get_all_namespace_ids();
        // Loop over all namespaces
        for namespace_id in &namespaces {
            // Get the global scope of the namespace
            let global_scope = self.prog_ctx.scope_registry.get_global_scope(namespace_id);

            // Loop over the inputs, outputs and states in declaration order and load them
            for var_id in global_scope.variables.iter() {
                if let Some(scope_var) = self.prog_ctx.scope_registry.get_var(var_id) {
                    match scope_var.var_kind {
                        VariableKind::Input { .. } => {
                            if let Some(item) = blueprint.get_item(var_id) {
                                let offset = Offset::PointerScaled(input_count);
                                self.load_input(params.input_ptr_ptr, item, offset, iteration);
                            }
                        }
                        VariableKind::Output => {
                            if let Some(item) = blueprint.get_item(var_id) {
                                self.init_output(item);
                            }
                        }
                        VariableKind::State => {
                            if let Some(item) = blueprint.get_item(var_id) {
                                let offset = Offset::PointerScaled(state_count);
                                self.load_or_init_state(
                                    params.state_ptr_ptr,
                                    is_first_and_should_init,
                                    item,
                                    offset,
                                );
                            }
                        }
                        VariableKind::GlobalConst => {
                            let var = self.declare_var(*var_id, &scope_var.value_type);
                            // Constants must have a default value
                            let def_val = self.translate_expr(scope_var.def_val.as_ref().unwrap());
                            self.builder.assign(var, def_val);
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    fn load_input(
        &mut self,
        ptr_ptr: Value,
        input_item: &BlueprintItem,
        input_offset: Offset,
        iteration: Option<Value>,
    ) {
        // Load the blueprint item value
        let val = self.load_blueprint_item(ptr_ptr, input_item, input_offset, iteration);
        // Register the translated variable
        self.register_translated_var(input_item.id, input_item.value_type, val);
    }

    fn init_output(&mut self, output_item: &BlueprintItem) {
        let output_var = self.declare_var(output_item.id, &output_item.value_type);
        // Output variables must have a default value
        let def_val = self.translate_expr(&output_item.def_val);
        self.builder.assign(output_var, def_val);
    }

    /// Initializes the state variables with the default value if should_init is true,
    /// and otherwise load the value from memory
    fn load_or_init_state(
        &mut self,
        ptr_ptr: Value,
        should_init: Value,
        state_item: &BlueprintItem,
        state_offset: Offset,
    ) {
        // Load the value from memory
        // Don't need to pass the iteration idex because state variables are not buffer even in buffer mode
        let loaded_val = self.load_blueprint_item(ptr_ptr, state_item, state_offset, None);
        // Get the default value of the state
        let def_val = self.translate_expr(&state_item.def_val);

        // Conditionally select whether to use the default value or the loaded value
        let val = self.builder.select(should_init, def_val, loaded_val);

        // Register the variable with the value
        self.register_translated_var(state_item.id, state_item.value_type, val);
    }

    // --- LOAD HELPER ---

    fn register_translated_var(
        &mut self,
        var_id: VariableID,
        var_type: ResolvedType,
        value: Value,
    ) {
        // Declare the variable
        let var = self.declare_var(var_id, &var_type);
        // Register the variable to the variables
        self.scope_registry.add_var(var_id, var);
        // Set the value to the variable
        self.builder.assign(var, value);
    }
}
