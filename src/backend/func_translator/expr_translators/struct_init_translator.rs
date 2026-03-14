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

use crate::{StructID, backend::func_translator::FuncTranslator};
use cranelift::prelude::{InstBuilder, StackSlotData, StackSlotKind};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn translate_struct_init(&mut self, struct_id: &StructID) -> Option<ir::Value> {
        // Store the value in the stack slot
        let Some(struct_decl) = self.comp_state.type_registry.get_struct(struct_id) else {
            return None;
        };

        // Create a stack slot
        let slot_data = StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            struct_decl.total_size,
            struct_decl.alignment as u8,
        );
        let slot = self.builder.func.create_sized_stack_slot(slot_data);
        // Store the fields to the slot
        for (field, offset) in struct_decl.fields.iter().zip(&struct_decl.field_offsets) {
            let translated_def_val = self.translate_expr(&field.def_val)?;
            self.builder
                .ins()
                .stack_store(translated_def_val, slot, *offset);
        }
        // Return the address to the struct
        let addr = self
            .builder
            .ins()
            .stack_addr(self.type_converter.pointer_type(), slot, 0);
        Some(addr)
    }
}
