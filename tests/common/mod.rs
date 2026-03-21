pub mod assert;
pub mod builders;

use kasl::{
    CompilationData, ParserDeclStmt,
    backend::Backend,
    blueprint_builder::BlueprintBuilder,
    builtin::BuiltinRegistry,
    compilation_data::{CompilerState, ProgramContext},
    error::{ErrorCollector, ErrorRecord},
    global_decl_collection::GlobalDeclCollector,
    kasl_parser,
    scope_graph_analyzing::ScopeGraphAnalyzer,
    scope_manager::IOBlueprint,
    statement_building::StatementBuilder,
    struct_graph_analyzing::StructGraphAnalyzer,
};
use std::mem;

pub struct TestContext {
    pub ec: ErrorCollector,
    pub prog_ctx: ProgramContext,
    pub comp_data: CompilationData,
    pub builtin_registry: BuiltinRegistry,
    pub comp_state: CompilerState,
}

impl Default for TestContext {
    fn default() -> Self {
        let mut test_ctx = Self {
            ec: ErrorCollector::default(),
            prog_ctx: ProgramContext::default(),
            comp_data: CompilationData::default(),
            builtin_registry: BuiltinRegistry::default(),
            comp_state: CompilerState::default(),
        };
        let root_namespace_id = test_ctx.prog_ctx.namespace_registry.get_root_namespace_id();
        test_ctx
            .prog_ctx
            .scope_registry
            .create_global_scope(root_namespace_id);
        test_ctx
    }
}

pub fn parse_expr(input: &str) -> Vec<ParserDeclStmt> {
    kasl_parser::parse(input).unwrap()
}

pub fn collect_global_decls(
    test_ctx: &mut TestContext,
    statements: &[ParserDeclStmt],
) -> Result<(), Vec<ErrorRecord>> {
    let root_namespace_id = test_ctx.prog_ctx.namespace_registry.get_root_namespace_id();
    let mut global_decl_collector = GlobalDeclCollector::new(
        &mut test_ctx.ec,
        &mut test_ctx.prog_ctx,
        &mut test_ctx.comp_data,
        &test_ctx.comp_state,
        &test_ctx.builtin_registry,
        root_namespace_id,
    );
    global_decl_collector.process(statements);
    test_ctx.ec.as_result()
}

pub fn analyze_structs(test_ctx: &mut TestContext) -> Result<(), Vec<ErrorRecord>> {
    let mut struct_graph_analyzer = StructGraphAnalyzer::new(
        &mut test_ctx.ec,
        &test_ctx.prog_ctx,
        &test_ctx.comp_data.struct_graph,
    );
    struct_graph_analyzer.analyze_all();
    test_ctx.ec.as_result()
}

pub fn build_stmts(test_ctx: &mut TestContext) -> Result<(), Vec<ErrorRecord>> {
    let mut stmt_builder = StatementBuilder::new(
        &mut test_ctx.ec,
        &mut test_ctx.prog_ctx,
        &mut test_ctx.comp_data,
        &test_ctx.builtin_registry,
    );
    stmt_builder.build_all();
    test_ctx.ec.as_result()
}

pub fn analyze_scopes(test_ctx: &mut TestContext) -> Result<(), Vec<ErrorRecord>> {
    let mut scope_graph_analyzer = ScopeGraphAnalyzer::new(
        &mut test_ctx.ec,
        &test_ctx.prog_ctx,
        &mut test_ctx.comp_data.scope_graph,
    );
    scope_graph_analyzer.analyze_all();
    test_ctx.ec.as_result()
}

pub fn build_blueprint(test_ctx: &mut TestContext) -> IOBlueprint {
    let blueprint_builder = BlueprintBuilder::new(&test_ctx.prog_ctx);
    blueprint_builder.build()
}

pub fn execute_program(
    test_ctx: &mut TestContext,
    blueprint: &IOBlueprint,
    inputs: &[*mut ()],
    outputs: &[*mut ()],
    states: &[*mut ()],
) {
    let mut backend = Backend::default();
    let root_namespace_id = test_ctx.prog_ctx.namespace_registry.get_root_namespace_id();
    let main_func_id = test_ctx
        .prog_ctx
        .func_ctx
        .get_global_func_id(root_namespace_id, "main")
        .unwrap();
    let code = backend
        .compile_once(
            &test_ctx.prog_ctx,
            &test_ctx.builtin_registry,
            blueprint,
            &main_func_id,
        )
        .unwrap();

    unsafe {
        run_code(code, inputs.as_ptr(), outputs.as_ptr(), states.as_ptr());
    }
}

unsafe fn run_code(
    code_ptr: *const u8,
    input: *const *mut (),
    output: *const *mut (),
    state: *const *mut (),
) {
    unsafe {
        let code_fn: fn(*const *mut (), *const *mut (), *const *mut ()) = mem::transmute(code_ptr);
        code_fn(input, output, state)
    }
}
