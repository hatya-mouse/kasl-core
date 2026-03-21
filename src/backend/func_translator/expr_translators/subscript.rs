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
        let base_ptr = self.translate_expr(lhs).unwrap();
        // Calculate the pointer to the corresponding value
        let val_ptr = self.calculate_array_offset(&lhs.value_type, base_ptr, index);

        match item_type {
            ResolvedType::Primitive(_) => {
                self.builder
                    .ins()
                    .load(translated_type, MemFlags::new(), val_ptr, 0)
            }
            ResolvedType::Struct(_) | ResolvedType::Array(_) => val_ptr,
        }
    }
}
