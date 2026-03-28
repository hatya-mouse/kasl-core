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
    backend::func_translator::{FuncTranslator, TranslatorParams},
};
use cranelift::prelude::{InstBuilder, IntCC, types};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn load_blueprint_access(
        &mut self,
        params: &TranslatorParams,
        blueprint: &IOBlueprint,
        iteration_index: Option<ir::Value>,
    ) {
        // Get the type of a pointer
        let pointer_type = self.type_converter.pointer_type();

        // Assume that the inputs, outputs and states are packed in the order they are declared
        let mut input_offset: i32 = 0;
        let mut state_offset: i32 = 0;

        // Calculate whether this is the first iteration and should be initialized
        let i32_zero = self.builder.ins().iconst(types::I32, 0);
        let is_first = iteration_index
            .map(|index| self.builder.ins().icmp(IntCC::Equal, index, i32_zero))
            .unwrap_or_else(|| self.builder.ins().iconst(types::I8, 1));
        let is_first_and_should_init = self.builder.ins().band(is_first, params.should_init);

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
                                self.load_input(
                                    pointer_type,
                                    params.input_ptr_ptr,
                                    item,
                                    input_offset,
                                    iteration_index,
                                );
                                input_offset += pointer_type.bytes() as i32;
                            }
                        }
                        VariableKind::Output => {
                            if let Some(item) = blueprint.get_item(var_id) {
                                self.init_output(item);
                            }
                        }
                        VariableKind::State => {
                            if let Some(item) = blueprint.get_item(var_id) {
                                self.load_or_init_state(
                                    pointer_type,
                                    params.state_ptr_ptr,
                                    is_first_and_should_init,
                                    item,
                                    state_offset,
                                );
                                state_offset += pointer_type.bytes() as i32;
                            }
                        }
                        VariableKind::GlobalConst => {
                            let var = self.declare_var(*var_id, &scope_var.value_type);
                            // Constants must have a default value
                            let translated_def_val = self
                                .translate_expr(scope_var.def_val.as_ref().unwrap())
                                .unwrap();
                            self.builder.def_var(var, translated_def_val);
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    fn load_input(
        &mut self,
        pointer_type: ir::Type,
        ptr_ptr: ir::Value,
        input_item: &BlueprintItem,
        input_offset: i32,
        iteration_index: Option<ir::Value>,
    ) {
        // Pass the optional sample index the buffer index in buffer mode
        let val = self.load_blueprint_item(
            pointer_type,
            ptr_ptr,
            input_item,
            input_offset,
            iteration_index,
        );
        self.register_translated_var(input_item.id, input_item.value_type, val);
    }

    fn init_output(&mut self, output_item: &BlueprintItem) {
        let output_var = self.declare_var(output_item.id, &output_item.value_type);
        // Output variables must have a default value
        let def_val = self.translate_expr(&output_item.def_val).unwrap();
        self.builder.def_var(output_var, def_val);
    }

    /// Initialize the variables with the default value if should_init is true,
    /// and otherwise load the value from memory
    fn load_or_init_state(
        &mut self,
        pointer_type: ir::Type,
        ptr_ptr: ir::Value,
        should_init: ir::Value,
        state_item: &BlueprintItem,
        state_offset: i32,
    ) {
        // Load the value from memory
        // Don't need to pass the iteration index because state variables are not buffer even in buffer mode
        let loaded_val =
            self.load_blueprint_item(pointer_type, ptr_ptr, state_item, state_offset, None);
        // Get the default value for the state
        let translated_def_val = self.translate_expr(&state_item.def_val).unwrap();

        // Conditionally select the default value or the loaded value
        let value = self
            .builder
            .ins()
            .select(should_init, translated_def_val, loaded_val);

        // Register the variable with the value
        self.register_translated_var(state_item.id, state_item.value_type, value);
    }

    // --- LOAD HELPERS ---

    fn register_translated_var(
        &mut self,
        var_id: VariableID,
        var_type: ResolvedType,
        value: ir::Value,
    ) {
        // Declare the variable
        let var = self.declare_var(var_id, &var_type);
        // Register the variable to the variables
        self.scope_registry.add_var(var_id, var);
        // Define the variable
        self.builder.def_var(var, value);
    }
}
