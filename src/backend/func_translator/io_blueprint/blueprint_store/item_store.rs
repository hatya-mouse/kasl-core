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

use crate::{
    ast::{scope_manager::BlueprintItem, type_registry::ResolvedType},
    backend::func_translator::FuncTranslator,
};
use cranelift::prelude::{InstBuilder, MemFlags, types};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub(super) fn store_blueprint_item(
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

    pub(super) fn store_value(
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
            ResolvedType::Array(array_id) => {
                self.copy_array(array_id, val, ptr, 0, offset);
            }
            ResolvedType::Struct(struct_id) => {
                self.copy_struct(struct_id, val, ptr, 0, offset);
            }
        }
    }
}
