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
    StructID, VariableID,
    backend::func_translator::{FuncTranslator, TranslatorParams},
    scope_manager::{BlueprintItem, BlueprintItemKind, IOBlueprint},
    type_registry::ResolvedType,
};
use cranelift::prelude::{InstBuilder, MemFlags, StackSlotData, StackSlotKind, types};
use cranelift_codegen::ir::{self, StackSlot};

impl FuncTranslator<'_> {
    pub fn load_blueprint_access(
        &mut self,
        params: &TranslatorParams,
        blueprint: &IOBlueprint,
        sample_index: Option<ir::Value>,
    ) {
        // Get the type of a pointer
        let pointer_type = self.type_converter.pointer_type();

        // Assume that the inputs, outputs and states are packed in the order they are declared
        let mut input_offset: i32 = 0;
        let mut state_offset: i32 = 0;

        // Loop over the inputs, outputs and states in declaration order and load them
        for (var_id, item_kind) in blueprint.get_order() {
            if let Some(item) = blueprint.get_item(var_id) {
                match item_kind {
                    BlueprintItemKind::Input => {
                        self.load_input(
                            pointer_type,
                            params.input_ptr_ptr,
                            item,
                            input_offset,
                            sample_index,
                        );
                        input_offset += pointer_type.bytes() as i32;
                    }
                    BlueprintItemKind::Output => {
                        self.init_output(item);
                    }
                    BlueprintItemKind::State => {
                        self.load_or_init_state(
                            pointer_type,
                            params.state_ptr_ptr,
                            params.should_init,
                            item,
                            state_offset,
                        );
                        state_offset += pointer_type.bytes() as i32;
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
        sample_index: Option<ir::Value>,
    ) {
        // Pass the optional sample index the buffer index in buffer mode
        let val = self.load_blueprint_item(
            pointer_type,
            ptr_ptr,
            input_item,
            input_offset,
            sample_index,
        );
        self.register_translated_var(input_item.id, input_item.value_type, val);
    }

    fn init_output(&mut self, output_item: &BlueprintItem) {
        let output_var = self.declare_var(output_item.id, &output_item.value_type);
        // Output variables must have a default value
        let def_val = self.translate_expr(&output_item.def_val);
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
        let loaded_val =
            self.load_blueprint_item(pointer_type, ptr_ptr, state_item, state_offset, None);
        // Get the default value for the state
        let translated_def_val = self.translate_expr(&state_item.def_val);

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

    fn load_blueprint_item(
        &mut self,
        pointer_type: ir::Type,
        ptr_ptr: ir::Value,
        item: &BlueprintItem,
        offset: i32,
        sample_index: Option<ir::Value>,
    ) -> ir::Value {
        // Get the pointer to the value by the pointer to the pointers
        let val_ptr = self
            .builder
            .ins()
            .load(pointer_type, MemFlags::new(), ptr_ptr, offset);

        let val_ptr = if let Some(i) = sample_index {
            // Calculate the pointer offset if it is in the buffer mode
            let item_size = self
                .builder
                .ins()
                .iconst(types::I32, item.actual_size as i64);
            let byte_offset = self.builder.ins().imul(i, item_size);
            // Extend the type to the pointer type
            let ptr_type_offset = self.builder.ins().uextend(pointer_type, byte_offset);
            self.builder.ins().iadd(val_ptr, ptr_type_offset)
        } else {
            val_ptr
        };

        // Load the value
        self.load_value(&item.value_type, val_ptr)
    }

    fn load_value(&mut self, value_type: &ResolvedType, ptr: ir::Value) -> ir::Value {
        match value_type {
            ResolvedType::Primitive(_) => self.builder.ins().load(
                self.type_converter.convert(value_type),
                MemFlags::new(),
                ptr,
                0,
            ),
            ResolvedType::Struct(struct_id) => {
                // Store the value in the stack slot
                let struct_decl = self.prog_ctx.type_registry.get_struct(struct_id).unwrap();

                // Create a stack slot
                let slot_data = StackSlotData::new(
                    StackSlotKind::ExplicitSlot,
                    struct_decl.total_size,
                    struct_decl.alignment,
                );
                let slot = self.builder.func.create_sized_stack_slot(slot_data);

                // Load and struct the value in the stack slot
                self.load_struct(struct_id, ptr, slot, 0);

                // Return the address to the struct
                self.builder
                    .ins()
                    .stack_addr(self.type_converter.pointer_type(), slot, 0)
            }
        }
    }

    fn load_struct(
        &mut self,
        struct_id: &StructID,
        val_ptr: ir::Value,
        stack_slot: StackSlot,
        offset: i32,
    ) {
        let struct_type = self.prog_ctx.type_registry.get_struct(struct_id).unwrap();
        for (field, field_offset) in struct_type
            .fields
            .iter()
            .zip(struct_type.field_offsets.iter().copied())
        {
            match &field.value_type {
                ResolvedType::Primitive(_) => {
                    let ir_type = self.type_converter.convert(&field.value_type);
                    let val =
                        self.builder
                            .ins()
                            .load(ir_type, MemFlags::new(), val_ptr, field_offset);
                    self.builder
                        .ins()
                        .stack_store(val, stack_slot, offset + field_offset);
                }
                ResolvedType::Struct(struct_id) => {
                    let child_offset = offset + field_offset;
                    self.load_struct(struct_id, val_ptr, stack_slot, child_offset);
                }
            }
        }
    }
}
