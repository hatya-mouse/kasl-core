use crate::{
    StructID, backend::func_translator::FuncTranslator, namespace_registry::ArrayID,
    type_registry::ResolvedType,
};
use cranelift::prelude::{InstBuilder, MemFlags};
use cranelift_codegen::ir;
use cranelift_module::Module;

impl FuncTranslator<'_> {
    pub fn copy_struct(
        &mut self,
        struct_id: &StructID,
        src: ir::Value,
        dst: ir::Value,
        src_offset: i32,
        base_offset: i32,
    ) {
        // Get the size of the struct from the type registry
        let struct_decl = self.prog_ctx.type_registry.get_struct(struct_id).unwrap();
        let struct_size = struct_decl.total_size as u64;

        // Copy the struct to the destination pointer
        self.copy_with_offset(struct_size, src, dst, src_offset, base_offset);
    }

    pub fn copy_array(
        &mut self,
        array_id: &ArrayID,
        src: ir::Value,
        dst: ir::Value,
        src_offset: i32,
        base_offset: i32,
    ) {
        // Calculate the size of the array in bytes
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
        let array_size = item_size as u64 * *array_decl.count() as u64;

        // Copy the array to the destination pointer
        self.copy_with_offset(array_size, src, dst, src_offset, base_offset);
    }

    fn copy_with_offset(
        &mut self,
        total_size: u64,
        src: ir::Value,
        dst: ir::Value,
        src_offset: i32,
        base_offset: i32,
    ) {
        // Calculate the src and dst ptr by adding the offset
        let src_ptr = if src_offset != 0 {
            let offset = self
                .builder
                .ins()
                .iconst(self.type_converter.pointer_type(), src_offset as i64);
            self.builder.ins().iadd(src, offset)
        } else {
            src
        };

        let dst_ptr = if base_offset != 0 {
            let offset = self
                .builder
                .ins()
                .iconst(self.type_converter.pointer_type(), base_offset as i64);
            self.builder.ins().iadd(dst, offset)
        } else {
            dst
        };

        // Copy the source to the destination pointer
        self.builder.emit_small_memory_copy(
            self.module.target_config(),
            dst_ptr,
            src_ptr,
            total_size,
            1,
            1,
            true,
            MemFlags::new(),
        );
    }
}
