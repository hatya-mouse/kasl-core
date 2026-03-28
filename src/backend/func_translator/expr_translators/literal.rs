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
    ast::type_registry::{PrimitiveType, ResolvedType},
    backend::func_translator::FuncTranslator,
};
use cranelift::prelude::InstBuilder;
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub(super) fn translate_int_literal(&mut self, value: u32) -> ir::Value {
        self.builder.ins().iconst(
            self.type_converter
                .convert(&ResolvedType::Primitive(PrimitiveType::Int)),
            value as i64,
        )
    }

    pub(super) fn translate_float_literal(&mut self, value: f32) -> ir::Value {
        self.builder.ins().f32const(value)
    }

    pub(super) fn translate_bool_literal(&mut self, value: bool) -> ir::Value {
        self.builder.ins().iconst(
            self.type_converter
                .convert(&ResolvedType::Primitive(PrimitiveType::Bool)),
            value as i64,
        )
    }
}
