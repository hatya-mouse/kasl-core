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

use kasl_ir::{Const, IRType, InstBuilder, IntBinOp, Offset, Value};

use crate::{
    ast_nodes::{scope_manager::BlueprintItem, type_registry::ResolvedType},
    lowerer::func_translator::{FuncTranslator, type_converter::convert_type},
};

impl FuncTranslator<'_> {
    pub(super) fn load_blueprint_item(
        &mut self,
        ptr_ptr: Value,
        item: &BlueprintItem,
        offset: Offset,
        iteration: Option<Value>,
    ) -> Value {
        // Get the pointer to the value by the pointer to the pointers
        let val_ptr = self.builder.load(IRType::Ptr, ptr_ptr, offset);

        let val_ptr = if let Some(i) = iteration {
            // Calculate the pointer offset if it is in the buffer mode
            let item_size = self.builder.const_val(Const::I32(item.actual_size as i32));
            let byte_offset = self.builder.ibop(IntBinOp::Mul, i, item_size);
            // Extend the type to the pointer type
            let ptr_type_offset = self.builder.iresize(byte_offset, IRType::Ptr);
            self.builder.ibop(IntBinOp::Add, val_ptr, ptr_type_offset)
        } else {
            val_ptr
        };

        // Load the value
        self.load_value(&item.value_type, val_ptr)
    }

    fn load_value(&mut self, value_type: &ResolvedType, ptr: Value) -> Value {
        match value_type {
            ResolvedType::Primitive(_) => {
                self.builder
                    .load(convert_type(value_type), ptr, Offset::zero())
            }
            ResolvedType::Array(array_id) => {
                let dst_ptr = self.alloc_array(array_id);
                // Copy the value to the destination pointer
                self.copy_array(array_id, ptr, Offset::zero(), dst_ptr, Offset::zero());
                dst_ptr
            }
            ResolvedType::Struct(struct_id) => {
                let dst_ptr = self.alloc_struct(struct_id);
                // Copy the value to the destination pointer
                self.copy_struct(struct_id, ptr, Offset::zero(), dst_ptr, Offset::zero());
                dst_ptr
            }
        }
    }
}
