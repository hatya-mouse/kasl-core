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

use crate::{Expr, backend::func_translator::FuncTranslator, type_registry::ResolvedType};
use cranelift::prelude::{InstBuilder, StackSlotData, StackSlotKind};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn translate_array_literal(&mut self, array_expr: &Expr) -> ir::Value {
        // Assume the type is array
        let array_id = match array_expr.value_type {
            ResolvedType::Array(array_id) => array_id,
            _ => unreachable!(),
        };

        // Get the total size and the alignment of the array
        let total_size = self
            .prog_ctx
            .type_registry
            .get_type_actual_size(&array_expr.value_type)
            .unwrap();
        let alignment = self
            .prog_ctx
            .type_registry
            .get_type_alignment(&array_expr.value_type)
            .unwrap();

        // Create a stack slot
        let slot_data =
            StackSlotData::new(StackSlotKind::ExplicitSlot, total_size as u32, alignment);
        let slot = self.builder.func.create_sized_stack_slot(slot_data);

        // Store the array items to the slot
        self.store_array_to_slot(&array_expr, &array_id, slot, 0);

        // Return the address to the array
        self.builder
            .ins()
            .stack_addr(self.type_converter.pointer_type(), slot, 0)
    }
}
