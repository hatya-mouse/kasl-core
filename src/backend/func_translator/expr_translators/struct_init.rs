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
use cranelift::prelude::InstBuilder;
use cranelift_codegen::ir::{self};

impl FuncTranslator<'_> {
    pub(super) fn translate_struct_init(&mut self, struct_id: &StructID) -> ir::Value {
        // Create a new stack slot
        let slot = self.alloc_struct(struct_id);

        // Store the fields to the slot
        self.store_init_fields_to_slot(struct_id, slot, 0);

        // Return the address to the struct
        self.builder
            .ins()
            .stack_addr(self.type_converter.pointer_type(), slot, 0)
    }
}
