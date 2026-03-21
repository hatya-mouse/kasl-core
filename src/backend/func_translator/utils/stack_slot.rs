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

use crate::{StructID, backend::func_translator::FuncTranslator, namespace_registry::ArrayID};
use cranelift::prelude::{StackSlotData, StackSlotKind};
use cranelift_codegen::ir::StackSlot;

impl FuncTranslator<'_> {
    pub fn alloc_struct(&mut self, struct_id: &StructID) -> StackSlot {
        let struct_decl = self.prog_ctx.type_registry.get_struct(struct_id).unwrap();

        // Create a stack slot
        let slot_data = StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            struct_decl.total_size,
            struct_decl.alignment,
        );
        self.builder.func.create_sized_stack_slot(slot_data)
    }

    pub fn alloc_array(&mut self, array_id: &ArrayID) -> StackSlot {
        let array_decl = self
            .prog_ctx
            .type_registry
            .get_array_decl(array_id)
            .unwrap();

        // Get the total size and the alignment of the array
        let item_size = self
            .prog_ctx
            .type_registry
            .get_type_actual_size(array_decl.item_type())
            .unwrap() as u32;
        let total_size = item_size * array_decl.count();
        let alignment = self
            .prog_ctx
            .type_registry
            .get_type_alignment(array_decl.item_type())
            .unwrap();

        // Create a stack slot
        let slot_data =
            StackSlotData::new(StackSlotKind::ExplicitSlot, total_size as u32, alignment);
        self.builder.func.create_sized_stack_slot(slot_data)
    }
}
