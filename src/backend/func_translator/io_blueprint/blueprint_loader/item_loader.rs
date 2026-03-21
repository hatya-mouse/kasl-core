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
    backend::func_translator::FuncTranslator, scope_manager::BlueprintItem,
    type_registry::ResolvedType,
};
use cranelift::prelude::{InstBuilder, MemFlags, StackSlotData, StackSlotKind, types};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub(super) fn load_blueprint_item(
        &mut self,
        pointer_type: ir::Type,
        ptr_ptr: ir::Value,
        item: &BlueprintItem,
        offset: i32,
        iteration_index: Option<ir::Value>,
    ) -> ir::Value {
        // Get the pointer to the value by the pointer to the pointers
        let val_ptr = self
            .builder
            .ins()
            .load(pointer_type, MemFlags::new(), ptr_ptr, offset);

        let val_ptr = if let Some(i) = iteration_index {
            // Calculate the pointer offset if it is in the buffer mode
            let item_size = self
                .builder
                .ins()
                .iconst(types::I32, item.actual_size as i64);
            let byte_offset = self.builder.ins().imul(i, item_size);
            // Extend the type to the pointer type
            let ptr_type_offset = self.extend_to_ptr(types::I32, byte_offset);
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
            ResolvedType::Array(array_id) => {}
            ResolvedType::Struct(struct_id) => {
                let struct_decl = self.prog_ctx.type_registry.get_struct(struct_id).unwrap();

                // Create a stack slot
                let slot_data = StackSlotData::new(
                    StackSlotKind::ExplicitSlot,
                    struct_decl.total_size,
                    struct_decl.alignment,
                );
                let slot = self.builder.func.create_sized_stack_slot(slot_data);
                let stack_addr =
                    self.builder
                        .ins()
                        .stack_addr(self.type_converter.pointer_type(), slot, 0);

                // Load and struct the value in the stack slot
                self.copy_struct(struct_id, ptr, stack_addr, 0);

                stack_addr
            }
        }
    }
}
