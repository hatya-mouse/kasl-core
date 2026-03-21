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
        array_type: &ResolvedType,
        base_ptr: ir::Value,
        index_expr: &Expr,
    ) -> ir::Value {
        // Get the size of the item
        let ResolvedType::Array(array_id) = array_type else {
            unreachable!();
        };
        let array_decl = self
            .prog_ctx
            .type_registry
            .get_array_decl(array_id)
            .unwrap();
        let item_size = self
            .prog_ctx
            .type_registry
            .get_type_actual_size(array_decl.item_type())
            .unwrap();
        let array_count = array_decl.count();

        // Translate the index
        let translated_index = self.translate_expr(index_expr).unwrap();
        // Clamp the index by the max value and zero
        let max_index = self
            .builder
            .ins()
            .iconst(types::I32, (array_count - 1) as i64);
        let zero = self.builder.ins().iconst(types::I32, 0);
        let zero_clamped_index = self.builder.ins().smax(zero, translated_index);
        let clamped_index = self.builder.ins().umin(zero_clamped_index, max_index);

        // Calculate the offset
        let item_size_ir = self.builder.ins().iconst(types::I32, item_size as i64);
        let offset = self.builder.ins().imul(item_size_ir, clamped_index);
        // Extend the offset value to the pointer type
        let ptr_type_offset = self.extend_to_ptr(types::I32, offset);
        // Calculate the pointer to the value
        self.builder.ins().iadd(base_ptr, ptr_type_offset)
    }
}
