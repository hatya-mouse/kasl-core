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
    lowerer::func_translator::FuncTranslator,
};
use kasl_ir::{Const, IRType, InstBuilder, IntBinOp, Offset, Value};

impl FuncTranslator<'_> {
    pub(super) fn store_blueprint_item(
        &mut self,
        ptr_ptr: Value,
        item: &BlueprintItem,
        offset: Offset,
        iteration: Option<Value>,
    ) {
        // Get the pointer to store the value at
        let ptr = self.builder.load(IRType::Ptr, ptr_ptr, offset);

        let ptr = if let Some(i) = iteration {
            // Calculate the pointer offset if the iteration is given
            let item_size = self.builder.const_val(Const::I32(item.actual_size as i32));
            let byte_offset = self.builder.ibop(IntBinOp::Mul, i, item_size);
            // Extend the type to the pointer type
            let ptr_type_offset = self.builder.iresize(byte_offset, IRType::Ptr);
            self.builder.ibop(IntBinOp::Add, ptr, ptr_type_offset)
        } else {
            ptr
        };

        // Get the value to be stored
        let var = self.scope_registry.get_var(&item.id);
        let val = self.builder.load_var(var);

        // Store the value
        self.store_value_to_var(&item.value_type, val, ptr);
    }

    fn store_value_to_var(&mut self, value_type: &ResolvedType, val: Value, ptr: Value) {
        match value_type {
            ResolvedType::Primitive(_) => {
                self.builder.store(val, ptr, Offset::zero());
            }
            ResolvedType::Array(array_id) => {
                self.copy_array(array_id, val, Offset::zero(), ptr, Offset::zero());
            }
            ResolvedType::Struct(struct_id) => {
                self.copy_struct(struct_id, val, Offset::zero(), ptr, Offset::zero());
            }
        }
    }
}
