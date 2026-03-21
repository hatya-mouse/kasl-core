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

mod brif_loop;
mod copy_value;
mod stack_slot;

use crate::{Expr, backend::func_translator::FuncTranslator, type_registry::ResolvedType};
use cranelift::prelude::{InstBuilder, types};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn extend_to_ptr(&mut self, val_type: ir::Type, val: ir::Value) -> ir::Value {
        let ptr_type = self.type_converter.pointer_type();
        if val_type == ptr_type {
            val
        } else {
            self.builder.ins().uextend(ptr_type, val)
        }
    }

    pub fn calculate_array_offset(
        &mut self,
        item_type: &ResolvedType,
        base_ptr: ir::Value,
        index_expr: &Expr,
    ) -> ir::Value {
        // Translate the index
        let translated_index = self.translate_expr(index_expr);
        // Get the size of the item
        let item_size = self
            .prog_ctx
            .type_registry
            .get_type_actual_size(item_type)
            .unwrap();

        // Calculate the offset
        let item_size_ir = self.builder.ins().iconst(types::I32, item_size as i64);
        let offset = self.builder.ins().imul(item_size_ir, translated_index);
        // Extend the offset value to the pointer type
        let ptr_type_offset = self.extend_to_ptr(types::I32, offset);
        // Calculate the pointer to the value
        self.builder.ins().iadd(base_ptr, ptr_type_offset)
    }
}
