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
use cranelift::prelude::{InstBuilder, MemFlags};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub(super) fn translate_subscript(
        &mut self,
        item_type: &ResolvedType,
        lhs: &Expr,
        index: &Expr,
    ) -> ir::Value {
        // Translate the value type
        let translated_type = self.type_converter.convert(item_type);

        // Translate the lhs into ir value
        let base_ptr = self.translate_expr(lhs);
        // Calculate the pointer to the corresponding value
        let val_ptr = self.calculate_array_offset(item_type, base_ptr, index);

        // Get the value depending on the type
        match item_type {
            ResolvedType::Primitive(_) => {
                self.builder
                    .ins()
                    .load(translated_type, MemFlags::new(), val_ptr, 0)
            }
            // Add offset to the array pointer
            ResolvedType::Array(_) => val_ptr,
            ResolvedType::Struct(_) => val_ptr,
        }
    }
}
