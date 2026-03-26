use crate::{
    CompilationData, MAIN_FUNCTION_NAME, ParserDeclStmt, Range,
    backend::Backend,
    blueprint_builder::BlueprintBuilder,
    builtin::BuiltinRegistry,
    compilation_data::{CompilerState, ProgramContext},
    error::{EK, ErrorCollector, ErrorKey, ErrorRecord, Ph, Pl, Sv},
    global_decl_collection::GlobalDeclCollector,
    kasl_parser,
    scope_graph_analyzing::ScopeGraphAnalyzer,
    scope_manager::IOBlueprint,
    statement_building::StatementBuilder,
    struct_graph_analyzing::StructGraphAnalyzer,
};
use std::{mem, path::PathBuf};

#[derive(Default)]
pub struct KaslCompiler {
    ec: ErrorCollector,
    pub prog_ctx: ProgramContext,
    comp_state: CompilerState,

    pub parser_decl_stmts: Vec<ParserDeclStmt>,
}

impl KaslCompiler {
    pub fn add_search_path(&mut self, path: PathBuf) {
        self.comp_state.child_search_paths.push(path);
    }

    pub fn set_search_paths(&mut self, paths: Vec<PathBuf>) {
        self.comp_state.child_search_paths = paths;
    }

    pub fn clear_search_paths(&mut self) {
        self.comp_state.child_search_paths.clear();
    }

    pub fn parse(&mut self, code: &str) -> Result<(), Box<ErrorRecord>> {
        self.parser_decl_stmts = kasl_parser::parse(code).map_err(|e| {
            Box::new(ErrorRecord::new(
                ErrorKey::new(
                    EK::ParserError,
                    Pl::StrVec(e.expected.tokens().map(|t| t.to_string()).collect()),
                ),
                Range::n(e.location.offset, e.location.offset),
                Ph::Parse,
                Sv::Error,
            ))
        })?;
        Ok(())
    }

    pub fn build(&mut self) -> Result<IOBlueprint, Vec<ErrorRecord>> {
        let mut comp_data = CompilationData::default();
        let builtin_registry = BuiltinRegistry::default();

        // 1. Collect global declarations
        let root_namespace = self.prog_ctx.namespace_registry.get_root_namespace_id();
        let mut global_decl_collector = GlobalDeclCollector::new(
            &mut self.ec,
            &mut self.prog_ctx,
            &mut comp_data,
            &self.comp_state,
            &builtin_registry,
            root_namespace,
        );
        global_decl_collector.process(&self.parser_decl_stmts);

        // 2. Analyze struct graph
        let mut struct_analyzer =
            StructGraphAnalyzer::new(&mut self.ec, &self.prog_ctx, &comp_data.struct_graph);
        struct_analyzer.analyze_all();

        // 3. Build statements
        let mut stmt_builder = StatementBuilder::new(
            &mut self.ec,
            &mut self.prog_ctx,
            &mut comp_data,
            &builtin_registry,
        );
        stmt_builder.build_all();

        // 4. Analyze scope graph
        let mut scope_analyzer =
            ScopeGraphAnalyzer::new(&mut self.ec, &self.prog_ctx, &mut comp_data.scope_graph);
        scope_analyzer.analyze_all();

        // 5. Build an IOBlueprint
        let blueprint_builder = BlueprintBuilder::new(&self.prog_ctx);
        let blueprint = blueprint_builder.build();

        self.ec.as_result().map(|_| blueprint)
    }

    pub fn compile_once(&mut self, blueprint: &IOBlueprint) -> Result<*const u8, Vec<ErrorRecord>> {
        let builtin_registry = BuiltinRegistry::default();

        // Compile the program
        let mut backend = Backend::default();
        let root_namespace_id = self.prog_ctx.namespace_registry.get_root_namespace_id();
        // Look up the main function, or return an error if it doesn't exist
        let main_func_id = self
            .prog_ctx
            .func_ctx
            .get_global_func_id(root_namespace_id, MAIN_FUNCTION_NAME)
            .ok_or_else(|| {
                vec![ErrorRecord::new(
                    ErrorKey::new(EK::NoMainFunc, Pl::None),
                    Range::zero(),
                    Ph::Backend,
                    Sv::Error,
                )]
            })?;

        let compiled = backend
            .compile_once(&self.prog_ctx, &builtin_registry, blueprint, &main_func_id)
            .map_err(|e| {
                vec![ErrorRecord::new(
                    ErrorKey::new(EK::CompilerBug, Pl::Str(e)),
                    Range::zero(),
                    Ph::Backend,
                    Sv::Error,
                )]
            })?;

        Ok(compiled)
    }

    pub fn compile_buffer(
        &mut self,
        blueprint: &IOBlueprint,
    ) -> Result<*const u8, Vec<ErrorRecord>> {
        let builtin_registry = BuiltinRegistry::default();

        // Compile the program
        let mut backend = Backend::default();
        let root_namespace_id = self.prog_ctx.namespace_registry.get_root_namespace_id();
        // Look up the main function, or return an error if it doesn't exist
        let main_func_id = self
            .prog_ctx
            .func_ctx
            .get_global_func_id(root_namespace_id, MAIN_FUNCTION_NAME)
            .ok_or_else(|| {
                vec![ErrorRecord::new(
                    ErrorKey::new(EK::NoMainFunc, Pl::None),
                    Range::zero(),
                    Ph::Backend,
                    Sv::Error,
                )]
            })?;

        let compiled = backend
            .compile_buffer(&self.prog_ctx, &builtin_registry, blueprint, &main_func_id)
            .map_err(|e| {
                vec![ErrorRecord::new(
                    ErrorKey::new(EK::CompilerBug, Pl::Str(e)),
                    Range::zero(),
                    Ph::Backend,
                    Sv::Error,
                )]
            })?;

        Ok(compiled)
    }
}
