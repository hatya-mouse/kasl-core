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
use cranelift::prelude::InstBuilder;
use cranelift_codegen::ir::StackSlot;

impl FuncTranslator<'_> {
    pub fn store_value_to_slot(&mut self, expr: &Expr, slot: StackSlot, offset: i32) {
        match expr.value_type {
            ResolvedType::Primitive(_) => {
                let val = self.translate_expr(expr).unwrap();
                self.builder.ins().stack_store(val, slot, offset);
            }
            ResolvedType::Array(array_id) => {
                self.store_array_to_slot(expr, &array_id, slot, offset)
            }
            ResolvedType::Struct(struct_id) => {
                self.store_init_fields_to_slot(&struct_id, slot, offset)
            }
        }
    }
}
