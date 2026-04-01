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
    ast::{StructID, namespace_registry::ArrayID},
    lowerer::func_translator::FuncTranslator,
};
use kasl_ir::{InstBuilder, Offset, Value};

impl FuncTranslator<'_> {
    pub(in crate::lowerer::func_translator) fn copy_struct(
        &mut self,
        struct_id: &StructID,
        src: Value,
        src_offset: Offset,
        dst: Value,
        dst_offset: Offset,
    ) {
        // Get the size of the struct from the type registry
        let struct_decl = self.prog_ctx.type_registry.get_struct(struct_id).unwrap();

        // Copy the struct to the destination pointer
        self.copy_with_offset(struct_decl.total_size, src, src_offset, dst, dst_offset);
    }

    pub(in crate::lowerer::func_translator) fn copy_array(
        &mut self,
        array_id: &ArrayID,
        src: Value,
        src_offset: Offset,
        dst: Value,
        dst_offset: Offset,
    ) {
        // Calculate the size of the array in bytes
        let array_decl = self
            .prog_ctx
            .type_registry
            .get_array_decl(array_id)
            .unwrap();
        let item_size = self
            .prog_ctx
            .type_registry
            .get_type_actual_size(array_decl.item_type())
            .unwrap();
        let array_size = item_size as u32 * *array_decl.count();

        // Copy the array to the destination pointer
        self.copy_with_offset(array_size, src, src_offset, dst, dst_offset);
    }

    fn copy_with_offset(
        &mut self,
        total_size: u32,
        src: Value,
        src_offset: Offset,
        dst: Value,
        dst_offset: Offset,
    ) {
        // Calculate the src and dst ptr by adding the offset
        let src_ptr = self.builder.ptr_add(src, src_offset);
        let dst_ptr = self.builder.ptr_add(dst, dst_offset);

        // Copy the source to the destination pointer
        self.builder
            .memcpy(total_size, src_ptr, Offset::zero(), dst_ptr, Offset::zero());
    }
}
