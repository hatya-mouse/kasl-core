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

use crate::{StructID, backend::func_translator::FuncTranslator, type_registry::ResolvedType};
use cranelift::prelude::InstBuilder;
use cranelift_codegen::ir::StackSlot;

impl FuncTranslator<'_> {
    pub fn store_init_fields_to_slot(
        &mut self,
        struct_id: &StructID,
        slot: StackSlot,
        base_offset: i32,
    ) {
        let struct_decl = self.prog_ctx.type_registry.get_struct(struct_id).unwrap();
        for (field, offset) in struct_decl
            .fields
            .iter()
            .zip(struct_decl.field_offsets.iter())
        {
            match field.value_type {
                ResolvedType::Primitive(_) => {
                    let val = self.translate_expr(&field.def_val).unwrap();
                    self.builder
                        .ins()
                        .stack_store(val, slot, base_offset + offset);
                }
                ResolvedType::Array(array_id) => {
                    self.store_array_to_slot(&field.def_val, &array_id, slot, base_offset);
                }
                ResolvedType::Struct(inner_id) => {
                    self.store_init_fields_to_slot(&inner_id, slot, base_offset + offset);
                }
            }
        }
    }
}
