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
