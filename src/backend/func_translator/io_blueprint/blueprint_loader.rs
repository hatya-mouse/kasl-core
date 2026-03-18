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
    VariableID,
    backend::func_translator::FuncTranslator,
    scope_manager::{BlueprintItem, IOBlueprint},
    type_registry::ResolvedType,
};
use cranelift::prelude::{InstBuilder, MemFlags};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn load_blueprint_access(
        &mut self,
        input_ptr_ptr: ir::Value,
        state_ptr_ptr: ir::Value,
        should_init: ir::Value,
        blueprint: &IOBlueprint,
    ) {
        // Get the type of a pointer
        let pointer_type = self.type_converter.pointer_type();

        // Loop over the inputs, outputs and states and load them
        self.load_inputs(pointer_type, input_ptr_ptr, blueprint);
        self.init_outputs(blueprint);
        self.load_or_init_states(pointer_type, state_ptr_ptr, should_init, blueprint);
    }

    fn load_inputs(&mut self, pointer_type: ir::Type, ptr_ptr: ir::Value, blueprint: &IOBlueprint) {
        let mut input_offset: usize = 0;
        for input_item in blueprint.get_inputs() {
            let val =
                self.load_blueprint_item(pointer_type, ptr_ptr, input_item, input_offset as i32);
            self.register_translated_var(input_item.id, input_item.value_type, val);
            // Increment the input offset by the size of a pointer
            input_offset += pointer_type.bytes() as usize;
        }
    }

    fn init_outputs(&mut self, blueprint: &IOBlueprint) {
        for output_item in blueprint.get_outputs() {
            let output_var = self.declare_var(output_item.id, &output_item.value_type);
            // Output variables must have a default value
            let def_val = self.translate_expr(&output_item.def_val);
            self.builder.def_var(output_var, def_val);
        }
    }

    /// Initialize the variables with the default value if should_init is true,
    /// and otherwise load the value from memory
    fn load_or_init_states(
        &mut self,
        pointer_type: ir::Type,
        ptr_ptr: ir::Value,
        should_init: ir::Value,
        blueprint: &IOBlueprint,
    ) {
        let mut state_offset: usize = 0;
        for state_item in blueprint.get_states() {
            // Load the value from memory
            let loaded_val =
                self.load_blueprint_item(pointer_type, ptr_ptr, state_item, state_offset as i32);
            // Get the default value for the state
            let translated_def_val = self.translate_expr(&state_item.def_val);

            // Conditionally select the default value or the loaded value
            let value = self
                .builder
                .ins()
                .select(should_init, translated_def_val, loaded_val);
            // Register the variable with the value
            self.register_translated_var(state_item.id, state_item.value_type, value);
            // Increment the state offset by the size of a pointer
            state_offset += pointer_type.bytes() as usize;
        }
    }

    // --- LOAD HELPERS ---

    fn load_blueprint_item(
        &mut self,
        pointer_type: ir::Type,
        ptr_ptr: ir::Value,
        item: &BlueprintItem,
        offset: i32,
    ) -> ir::Value {
        // Get the pointer to the value by the pointer to the pointers
        let ptr = self
            .builder
            .ins()
            .load(pointer_type, MemFlags::new(), ptr_ptr, offset);

        // Load the value
        self.builder.ins().load(
            self.type_converter.convert(&item.value_type),
            MemFlags::new(),
            ptr,
            0,
        )
    }

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
