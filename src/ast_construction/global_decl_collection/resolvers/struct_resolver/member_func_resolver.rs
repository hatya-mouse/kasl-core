use crate::{
    Range, StructID,
    global_decl_collection::{GlobalDeclCollector, resolvers::FuncDeclInfo},
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_member_func_decl(
        &mut self,
        struct_id: StructID,
        decl_range: Range,
        info: FuncDeclInfo<'_>,
    ) {
        // Build the function node
        let Some(func) = self.build_func(
            info.func_type,
            info.name,
            info.params,
            info.return_type,
            decl_range,
        ) else {
            return;
        };

        // Register the function
        let func_id = self.prog_ctx.func_ctx.register_member_func(func, struct_id);

        // Mark the function name as used in the namespace
        self.mark_name_used(info.name);

        // Register the function body to the func body map
        self.comp_data
            .func_body_map
            .register(func_id, info.body.to_vec());
    }
}
