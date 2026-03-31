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

use crate::{
    ast::{Expr, type_registry::ResolvedType},
    lowerer::func_translator::{FuncTranslator, type_converter::convert_type},
};
use kasl_ir::ir::{InstBuilder, Offset, Value};

impl FuncTranslator<'_> {
    pub(super) fn translate_subscript(
        &mut self,
        item_type: &ResolvedType,
        lhs: &Expr,
        index: &Expr,
    ) -> Value {
        // Translate the value type
        let translated_type = convert_type(item_type);

        // Translalte the lhs into an ir value
        let base_ptr = self.translate_expr(lhs).unwrap();
        // Calculate the pinter to the corresponding value
        let val_ptr = self.calculate_array_offset(&lhs.value_type, base_ptr, index);

        match item_type {
            ResolvedType::Primitive(_) => {
                self.builder.load(translated_type, val_ptr, Offset::zero())
            }
            ResolvedType::Struct(_) | ResolvedType::Array(_) => val_ptr,
        }
    }
}
