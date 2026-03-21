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
    backend::func_translator::{FuncTranslator, TranslatorParams},
    scope_manager::{BlueprintItem, IOBlueprint},
    type_registry::ResolvedType,
};
use cranelift::prelude::{InstBuilder, MemFlags, types};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn store_blueprint(
        &mut self,
        params: &TranslatorParams,
        blueprint: &IOBlueprint,
        iteration_index: Option<ir::Value>,
    ) {
        // Get the type of a pointer
        let pointer_type = self.type_converter.pointer_type();

        // Store outputs and states
        self.store_outputs(
            pointer_type,
            params.output_ptr_ptr,
            blueprint,
            iteration_index,
        );
        self.store_states(pointer_type, params.state_ptr_ptr, blueprint);
    }

    fn store_outputs(
        &mut self,
        pointer_type: ir::Type,
        ptr_ptr: ir::Value,
        blueprint: &IOBlueprint,
        iteration_index: Option<ir::Value>,
    ) {
        let mut output_offset: usize = 0;
        for output_item in blueprint.get_outputs() {
            self.store_blueprint_item(
                pointer_type,
                ptr_ptr,
                output_item,
                output_offset as i32,
                iteration_index,
            );
            // Increment the output offset by the size of a pointer
            // because each output is stored as a pointer to the actual value
            output_offset += pointer_type.bytes() as usize;
        }
    }

    fn store_states(
        &mut self,
        pointer_type: ir::Type,
        ptr_ptr: ir::Value,
        blueprint: &IOBlueprint,
    ) {
        let mut state_offset: usize = 0;
        for state_item in blueprint.get_states() {
            // States shares the memory over the loop so we do not need to pass a sample index
            self.store_blueprint_item(pointer_type, ptr_ptr, state_item, state_offset as i32, None);
            // Increment the state offset by the size of a pointer
            state_offset += pointer_type.bytes() as usize;
        }
    }

    // --- STORE HELPERS ---

    fn store_blueprint_item(
        &mut self,
        pointer_type: ir::Type,
        ptr_ptr: ir::Value,
        item: &BlueprintItem,
        offset: i32,
        iteration_index: Option<ir::Value>,
    ) {
        // Get the pointer to store the value at
        let ptr = self
            .builder
            .ins()
            .load(pointer_type, MemFlags::new(), ptr_ptr, offset);

        let ptr = if let Some(i) = iteration_index {
            // Calculate the pointer offset if it is in the buffer mode
            let item_size = self
                .builder
                .ins()
                .iconst(types::I32, item.actual_size as i64);
            let byte_offset = self.builder.ins().imul(i, item_size);
            // Extend the type to the pointer type
            let ptr_type_offset = self.extend_to_ptr(types::I32, byte_offset);
            self.builder.ins().iadd(ptr, ptr_type_offset)
        } else {
            ptr
        };

        // Get the value to be stored
        let var = self.scope_registry.get_var(&item.id);
        let val = self.builder.use_var(var);

        // Store the value
        self.store_value(&item.value_type, val, ptr, 0);
    }

    fn store_value(
        &mut self,
        value_type: &ResolvedType,
        val: ir::Value,
        ptr: ir::Value,
        offset: i32,
    ) {
        match value_type {
            ResolvedType::Primitive(_) => {
                self.builder.ins().store(MemFlags::new(), val, ptr, offset);
            }
            ResolvedType::Struct(struct_id) => {
                let struct_type = self.prog_ctx.type_registry.get_struct(struct_id).unwrap();
                for (field, field_offset) in struct_type
                    .fields
                    .iter()
                    .zip(struct_type.field_offsets.iter().copied())
                {
                    match &field.value_type {
                        ResolvedType::Primitive(_) => {
                            let ir_type = self.type_converter.convert(&field.value_type);
                            let field_val = self.builder.ins().load(
                                ir_type,
                                MemFlags::new(),
                                val,
                                field_offset,
                            );
                            self.builder.ins().store(
                                MemFlags::new(),
                                field_val,
                                ptr,
                                offset + field_offset,
                            );
                        }
                        ResolvedType::Struct(_) => {
                            let child_offset = offset + field_offset;
                            self.store_value(&field.value_type, val, ptr, child_offset);
                        }
                    }
                }
            }
        }
    }
}
