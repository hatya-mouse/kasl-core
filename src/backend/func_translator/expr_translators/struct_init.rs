use crate::{StructID, backend::func_translator::FuncTranslator};
use cranelift::prelude::InstBuilder;
use cranelift_codegen::ir::{self};

impl FuncTranslator<'_> {
    pub(super) fn translate_struct_init(&mut self, struct_id: &StructID) -> ir::Value {
        // Create a new stack slot
        let slot = self.alloc_struct(struct_id);

        // Store the fields to the slot
        self.store_init_fields_to_slot(struct_id, slot, 0);

        // Return the address to the struct
        self.builder
            .ins()
            .stack_addr(self.type_converter.pointer_type(), slot, 0)
    }
}
