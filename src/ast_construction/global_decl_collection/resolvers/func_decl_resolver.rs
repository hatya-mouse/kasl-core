use crate::{
    Range,
    error::Ph,
    global_decl_collection::{GlobalDeclCollector, resolvers::FuncDeclInfo},
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_global_func_decl(&mut self, info: FuncDeclInfo<'_>, decl_range: Range) {
        // Check if a function with the same name already exists
        if self.is_name_used(info.name) {
            self.ec
                .duplicate_name(decl_range, Ph::GlobalDeclCollection, info.name);
            return;
        }

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
        let func_id = self
            .prog_ctx
            .func_ctx
            .register_global_func(self.current_namespace, func);

        // Mark the function name as used in the namespace
        self.mark_name_used(info.name);

        // Register the function body to the function body map
        self.comp_data
            .func_body_map
            .register(func_id, info.body.to_vec());
    }
}
