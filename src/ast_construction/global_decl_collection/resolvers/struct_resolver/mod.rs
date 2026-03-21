mod member_func_resolver;
mod struct_field_resolver;

use crate::{
    ParserDeclStmt, ParserDeclStmtKind, Range, StructID,
    error::Ph,
    global_decl_collection::{GlobalDeclCollector, resolvers::FuncDeclInfo},
    symbol_table::FunctionType,
    type_registry::StructDecl,
};

impl<'a> GlobalDeclCollector<'a> {
    pub fn resolve_struct_decl(
        &mut self,
        name: &str,
        body: &'a [ParserDeclStmt],
        decl_range: Range,
    ) {
        // Check if the struct with the same name already exists
        if self.is_name_used(name) {
            self.ec
                .duplicate_name(decl_range, Ph::StructCollection, name);
        }

        // Generate a unique struct ID and create a new struct decl
        let struct_id = self.prog_ctx.type_registry.generate_struct_id();
        let mut struct_decl = StructDecl::new(name.to_string(), decl_range);
        self.resolve_struct_body(struct_id, &mut struct_decl, body);

        // Calculate the struct layout
        struct_decl.compute_layout(&self.prog_ctx.type_registry);

        // Register the struct in the type registry with a generated ID
        self.prog_ctx.type_registry.register_struct(
            self.current_namespace,
            struct_decl,
            name.to_string(),
            struct_id,
        );

        // Mark the struct name as used in the namespace
        self.mark_name_used(name);
    }

    fn resolve_struct_body(
        &mut self,
        struct_id: StructID,
        struct_decl: &mut StructDecl,
        body: &'a [ParserDeclStmt],
    ) {
        for stmt in body {
            match &stmt.kind {
                ParserDeclStmtKind::StructField {
                    name,
                    value_type,
                    def_val,
                } => self.resolve_struct_field(
                    struct_id,
                    struct_decl,
                    name,
                    value_type,
                    def_val,
                    stmt.range,
                ),

                ParserDeclStmtKind::FuncDecl {
                    is_static,
                    name,
                    params,
                    return_type,
                    body,
                } => {
                    let func_type = if *is_static {
                        FunctionType::Static
                    } else {
                        FunctionType::Instance(struct_id)
                    };

                    let info = FuncDeclInfo {
                        func_type,
                        name,
                        params,
                        return_type,
                        body,
                    };
                    self.resolve_member_func_decl(struct_id, stmt.range, info);
                }

                _ => {
                    self.ec.invalid_struct_stmt(
                        stmt.range,
                        Ph::GlobalDeclCollection,
                        stmt.kind.to_string(),
                    );
                }
            }
        }
    }
}
