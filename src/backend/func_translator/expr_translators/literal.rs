use crate::{
    backend::func_translator::FuncTranslator,
    type_registry::{PrimitiveType, ResolvedType},
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
