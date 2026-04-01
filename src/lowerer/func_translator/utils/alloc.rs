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

use kasl_ir::{InstBuilder, Value};

use crate::{
    ast::{StructID, namespace_registry::ArrayID},
    lowerer::func_translator::FuncTranslator,
};

impl FuncTranslator<'_> {
    /// Allocates memory for the given struct type and returns the allocated pointer.
    pub(in crate::lowerer::func_translator) fn alloc_struct(
        &mut self,
        struct_id: &StructID,
    ) -> Value {
        let struct_decl = self.prog_ctx.type_registry.get_struct(struct_id).unwrap();

        // Allocate memory
        self.builder
            .alloc(struct_decl.total_size, struct_decl.alignment)
    }

    /// Allocates memory for the given array type and returns the allocated pointer.
    pub(in crate::lowerer::func_translator) fn alloc_array(&mut self, array_id: &ArrayID) -> Value {
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
            .unwrap();
        let total_size = item_size * array_decl.count();
        let align = self
            .prog_ctx
            .type_registry
            .get_type_alignment(array_decl.item_type())
            .unwrap();

        // Allocate memory
        self.builder.alloc(total_size, align)
    }
}
