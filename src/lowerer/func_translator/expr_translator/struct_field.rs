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

use kasl_ir::ir::{InstBuilder, Offset, Value};

use crate::{
    ast::{Expr, type_registry::ResolvedType},
    lowerer::func_translator::{FuncTranslator, type_converter::convert_type},
};

impl FuncTranslator<'_> {
    pub(super) fn translate_struct_field_expr(
        &mut self,
        lhs: &Expr,
        value_type: &ResolvedType,
        offset: u32,
    ) -> Value {
        // Translate the expression
        let translated_lhs = self.translate_expr(lhs).unwrap();
        // Translate the type
        let translated_type = convert_type(value_type);

        // Get the value depending on the value
        match value_type {
            ResolvedType::Primitive(_) => {
                self.builder
                    .load(translated_type, translated_lhs, Offset::Immediate(offset))
            }
            // Add offset to the struct pointer
            ResolvedType::Array(_) => self
                .builder
                .ptr_add(translated_lhs, Offset::Immediate(offset)),
            ResolvedType::Struct(_) => self
                .builder
                .ptr_add(translated_lhs, Offset::Immediate(offset)),
        }
    }
}
