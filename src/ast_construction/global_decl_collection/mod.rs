mod resolvers;
mod stmt_process;

pub use resolvers::FuncDeclInfo;

use crate::{
    CompilationData, NameSpaceID, ParserDeclStmt, ScopeID,
    builtin::BuiltinRegistry,
    compilation_data::{CompilerState, ProgramContext},
    error::ErrorCollector,
};

pub struct GlobalDeclCollector<'a> {
    ec: &'a mut ErrorCollector,
    prog_ctx: &'a mut ProgramContext,
    comp_data: &'a mut CompilationData,
    comp_state: &'a CompilerState,
    builtin_registry: &'a BuiltinRegistry,

    current_namespace: NameSpaceID,
    global_scope_id: ScopeID,
}

impl<'a> GlobalDeclCollector<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        prog_ctx: &'a mut ProgramContext,
        comp_data: &'a mut CompilationData,
        comp_state: &'a CompilerState,
        builtin_registry: &'a BuiltinRegistry,
        current_namespace: NameSpaceID,
    ) -> Self {
        let global_scope_id = prog_ctx
            .scope_registry
            .get_global_scope_id(&current_namespace);

        Self {
            ec,
            prog_ctx,
            comp_data,
            comp_state,
            builtin_registry,
            current_namespace,
            global_scope_id,
        }
    }

    pub fn process(&mut self, decl_stmts: &'a [ParserDeclStmt]) {
        for stmt in decl_stmts.iter() {
            self.process_stmt(stmt);
        }
    }

    pub fn mark_name_used(&mut self, name: &str) {
        // Mark the name as used in the namespace
        self.prog_ctx
            .scope_registry
            .mark_name_used(&self.global_scope_id, name);
    }

    pub fn is_name_used(&self, name: &str) -> bool {
        self.prog_ctx
            .scope_registry
            .is_name_used(&self.global_scope_id, name)
    }
}
