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

use cranelift::prelude::{InstBuilder, MemFlags};
use cranelift_codegen::ir;

use crate::{Expr, backend::func_translator::FuncTranslator, type_registry::ResolvedType};

impl FuncTranslator<'_> {
    pub fn translate_struct_field_expr(
        &mut self,
        lhs: &Expr,
        value_type: &ResolvedType,
        offset: i32,
    ) -> ir::Value {
        // Translate the expression
        let translated_lhs = self.translate_expr(lhs);
        // Translate the type
        let translated_type = self.type_converter.convert(value_type);

        // Get the value depending on the type
        match value_type {
            ResolvedType::Primitive(_) => {
                self.builder
                    .ins()
                    .load(translated_type, MemFlags::new(), translated_lhs, offset)
            }
            // Add offset to the struct pointer
            ResolvedType::Array(_) => self.builder.ins().iadd_imm(translated_lhs, offset as i64),
            ResolvedType::Struct(_) => self.builder.ins().iadd_imm(translated_lhs, offset as i64),
        }
    }
}
