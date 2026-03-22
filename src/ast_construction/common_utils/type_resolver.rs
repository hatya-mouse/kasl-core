use crate::{
    NameSpaceID, compilation_data::ProgramContext, parser_ast::ParserTypeName,
    type_registry::ResolvedType,
};

pub(crate) fn resolve_type(
    current_namespace: NameSpaceID,
    prog_ctx: &mut ProgramContext,
    parser_type: &ParserTypeName,
) -> Option<ResolvedType> {
    match parser_type {
        ParserTypeName::SymbolPath(path) => {
            let (namespace_id, type_name) = prog_ctx
                .namespace_registry
                .resolve_namespace_from_path(current_namespace, path.clone());

            // Resolved the type name
            prog_ctx
                .type_registry
                .resolve_type_name(namespace_id, &type_name.to_string())
        }
        ParserTypeName::Array(item_type, count) => {
            let resolved_item_type = resolve_type(current_namespace, prog_ctx, item_type)?;

            // Register or get the array ID
            let array_id = prog_ctx
                .type_registry
                .register_or_get_array(resolved_item_type, *count);
            Some(ResolvedType::Array(array_id))
        }
    }
}
