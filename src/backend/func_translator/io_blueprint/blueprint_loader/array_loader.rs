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
    StructID, backend::func_translator::FuncTranslator, namespace_registry::ArrayID,
    type_registry::ResolvedType,
};
use cranelift::prelude::{InstBuilder, MemFlags};
use cranelift_codegen::ir::{self, StackSlot};

impl FuncTranslator<'_> {
    pub(super) fn load_array(
        &mut self,
        array_id: &ArrayID,
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
                ResolvedType::Array(array_id) => {
                    let child_offset = offset + field_offset;
                }
                ResolvedType::Struct(struct_id) => {
                    let child_offset = offset + field_offset;
                    self.load_struct(struct_id, val_ptr, stack_slot, child_offset);
                }
            }
        }
    }
}
