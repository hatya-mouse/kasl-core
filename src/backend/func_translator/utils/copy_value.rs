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
        let struct_decl = self.prog_ctx.type_registry.get_struct(struct_id).unwrap();
        let struct_size = struct_decl.total_size;

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

        // Copy the struct to the destination pointer
        self.builder.emit_small_memory_copy(
            self.module.target_config(),
            dst_ptr,
            src_ptr,
            struct_size as u64,
            1,
            1,
            true,
            MemFlags::new(),
        );
    }

    pub fn copy_array(
        &mut self,
        array_id: &ArrayID,
        src: ir::Value,
        dst: ir::Value,
        src_offset: i32,
        base_offset: i32,
    ) {
        let array_decl = self
            .prog_ctx
            .type_registry
            .get_array_decl(array_id)
            .unwrap();
        // Get the size of the item type from the type registry
        let item_type = array_decl.item_type();
        let item_size = self
            .prog_ctx
            .type_registry
            .get_type_actual_size(item_type)
            .unwrap() as i32;

        let mut offset: i32 = 0;
        for _ in 0..*array_decl.count() {
            self.copy_value(item_type, src, dst, src_offset, base_offset, offset);
            // Increment the offset every iteration
            offset += item_size;
        }
    }

    fn copy_value(
        &mut self,
        value_type: &ResolvedType,
        src: ir::Value,
        dst: ir::Value,
        src_offset: i32,
        base_offset: i32,
        offset: i32,
    ) {
        match value_type {
            ResolvedType::Primitive(_) => {
                let ir_type = self.type_converter.convert(value_type);
                let val =
                    self.builder
                        .ins()
                        .load(ir_type, MemFlags::new(), src, src_offset + offset);
                self.builder
                    .ins()
                    .store(MemFlags::new(), val, dst, base_offset + offset);
            }
            ResolvedType::Array(inner_id) => {
                self.copy_array(
                    inner_id,
                    src,
                    dst,
                    src_offset + offset,
                    base_offset + offset,
                );
            }
            ResolvedType::Struct(inner_id) => {
                self.copy_struct(
                    inner_id,
                    src,
                    dst,
                    src_offset + offset,
                    base_offset + offset,
                );
            }
        }
    }
}
