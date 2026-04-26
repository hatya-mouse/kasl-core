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
    ast_nodes::{StructID, type_registry::ResolvedType},
    lowerer::func_translator::FuncTranslator,
};
use kasl_ir::{InstBuilder, Offset, Value};

impl FuncTranslator<'_> {
    pub(in crate::lowerer::func_translator::expr_translator) fn store_init_fields(
        &mut self,
        struct_id: &StructID,
        dst_ptr: Value,
        dst_offset: u32,
    ) {
        let struct_decl = self.prog_ctx.type_registry.get_struct(struct_id).unwrap();
        for (field, field_offset) in struct_decl
            .fields
            .iter()
            .zip(struct_decl.field_offsets.iter())
        {
            match field.value_type {
                ResolvedType::Primitive(_) => {
                    let val = self.translate_expr(&field.def_val).unwrap();
                    self.builder
                        .store(val, dst_ptr, Offset::Immediate(dst_offset + field_offset));
                }
                ResolvedType::Array(array_id) => {
                    self.store_array(&field.def_val, &array_id, dst_ptr, dst_offset);
                }
                ResolvedType::Struct(struct_id) => {
                    self.store_init_fields(&struct_id, dst_ptr, dst_offset + field_offset);
                }
            }
        }
    }
}
