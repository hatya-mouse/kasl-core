use cranelift::prelude::{InstBuilder, MemFlags};
use cranelift_codegen::ir;

use crate::{Expr, backend::func_translator::FuncTranslator, type_registry::ResolvedType};

impl FuncTranslator<'_> {
    pub(super) fn translate_struct_field_expr(
        &mut self,
        lhs: &Expr,
        value_type: &ResolvedType,
        offset: i32,
    ) -> ir::Value {
        // Translate the expression
        let translated_lhs = self.translate_expr(lhs).unwrap();
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
