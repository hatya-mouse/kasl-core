use crate::{Expr, backend::func_translator::FuncTranslator, type_registry::ResolvedType};
use cranelift::prelude::InstBuilder;
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub(super) fn translate_array_literal(&mut self, array_expr: &Expr) -> ir::Value {
        // Assume the type is array
        let array_id = match array_expr.value_type {
            ResolvedType::Array(array_id) => array_id,
            _ => unreachable!(),
        };

        // Create a new stack slot
        let slot = self.alloc_array(&array_id);

        // Store the array items to the slot
        self.store_array_to_slot(array_expr, &array_id, slot, 0);

        // Return the address to the array
        self.builder
            .ins()
            .stack_addr(self.type_converter.pointer_type(), slot, 0)
    }
}
