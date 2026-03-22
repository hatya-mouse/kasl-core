use crate::{
    Range, common_utils::resolve_type, error::Ph, global_decl_collection::GlobalDeclCollector,
    parser_ast::ParserTypeName,
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_typealias(&mut self, alias: &str, target: &ParserTypeName, decl_range: Range) {
        // Resolve the target type
        let resolved_target = match resolve_type(self.current_namespace, self.prog_ctx, target) {
            Some(ty) => ty,
            None => {
                self.ec
                    .type_not_found(decl_range, Ph::GlobalDeclCollection, target.to_string());
                return;
            }
        };

        // Add the typeealias to the type registry
        self.prog_ctx.type_registry.register_typealias(
            self.current_namespace,
            alias.to_string(),
            resolved_target,
        );
        // Mark the typealias as used in the namespace
        self.mark_name_used(alias);
    }
}
