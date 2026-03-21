mod builders;
/// Builds a Block which contains ScopeID from a list of statements.
mod scope_block_builder;
mod stmt_builder;

use crate::{
    CompilationData, NameSpaceID, ScopeID, builtin::BuiltinRegistry,
    compilation_data::ProgramContext, error::ErrorCollector, type_registry::ResolvedType,
};

/// Builds a statements from raw parser statements.
/// Should not be reused across multiple blocks.
pub struct BlockStmtBuilder<'a> {
    ec: &'a mut ErrorCollector,
    prog_ctx: &'a mut ProgramContext,
    comp_data: &'a mut CompilationData,
    builtin_registry: &'a BuiltinRegistry,

    scope_id: ScopeID,
    namespace_id: NameSpaceID,
    expected_return_type: ResolvedType,
}

impl<'a> BlockStmtBuilder<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        prog_ctx: &'a mut ProgramContext,
        comp_data: &'a mut CompilationData,
        builtin_registry: &'a BuiltinRegistry,
        scope_id: ScopeID,
        namespace_id: NameSpaceID,
        expected_return_type: ResolvedType,
    ) -> Self {
        Self {
            ec,
            prog_ctx,
            comp_data,
            builtin_registry,
            scope_id,
            namespace_id,
            expected_return_type,
        }
    }

    pub fn mark_name_used(&mut self, name: &str) {
        // Mark the name as used in the namespace
        self.prog_ctx
            .scope_registry
            .mark_name_used(&self.scope_id, name);
    }

    pub fn is_name_used(&self, name: &str) -> bool {
        self.prog_ctx
            .scope_registry
            .is_name_used(&self.scope_id, name)
    }
}
