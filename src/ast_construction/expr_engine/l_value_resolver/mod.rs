mod recursive_resolver;

use crate::{
    CompilationData, NameSpaceID, ScopeID, builtin::BuiltinRegistry,
    compilation_data::ProgramContext, error::ErrorCollector,
};

pub struct LValueResolver<'a> {
    ec: &'a mut ErrorCollector,
    prog_ctx: &'a mut ProgramContext,
    comp_data: &'a mut CompilationData,
    builtin_registry: &'a BuiltinRegistry,
    current_scope: ScopeID,
    current_namespace: NameSpaceID,
}

impl<'a> LValueResolver<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        prog_ctx: &'a mut ProgramContext,
        comp_data: &'a mut CompilationData,
        builtin_registry: &'a BuiltinRegistry,
        current_scope: ScopeID,
        current_namespace: NameSpaceID,
    ) -> Self {
        Self {
            ec,
            prog_ctx,
            comp_data,
            builtin_registry,
            current_scope,
            current_namespace,
        }
    }
}
