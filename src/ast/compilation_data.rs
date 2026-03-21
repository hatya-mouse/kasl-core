use crate::{
    OperatorContext, ScopeRegistry,
    namespace_registry::NameSpaceRegistry,
    scope_manager::ScopeGraph,
    symbol_table::{FuncBodyMap, FunctionContext, OpBodyMap},
    type_registry::{StructGraph, TypeRegistry},
};
use std::{collections::HashSet, path::PathBuf};

#[derive(Debug)]
pub struct ProgramContext {
    pub func_ctx: FunctionContext,
    pub op_ctx: OperatorContext,
    pub scope_registry: ScopeRegistry,
    pub type_registry: TypeRegistry,
    pub namespace_registry: NameSpaceRegistry,
}

impl Default for ProgramContext {
    fn default() -> Self {
        let mut prog_ctx = Self {
            func_ctx: FunctionContext::default(),
            op_ctx: OperatorContext::default(),
            scope_registry: ScopeRegistry::default(),
            type_registry: TypeRegistry::default(),
            namespace_registry: NameSpaceRegistry::default(),
        };

        let root_namespace_id = prog_ctx.namespace_registry.get_root_namespace_id();
        prog_ctx
            .scope_registry
            .create_global_scope(root_namespace_id);
        prog_ctx
    }
}

#[derive(Debug, Default)]
pub struct CompilationData {
    pub func_body_map: FuncBodyMap,
    pub op_body_map: OpBodyMap,
    pub struct_graph: StructGraph,
    pub scope_graph: ScopeGraph,
}

#[derive(Debug, Default, Clone)]
pub struct CompilerState {
    pub child_search_paths: Vec<PathBuf>,
    pub imported_paths: HashSet<PathBuf>,
}
