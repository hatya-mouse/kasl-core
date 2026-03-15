//
// © 2025-2026 Shuntaro Kasatani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

pub mod builders;

use kasl::{
    CompilationData, NameSpace, ParserDeclStmt,
    backend::Backend,
    blueprint_builder::BlueprintBuilder,
    builtin::BuiltinRegistry,
    compilation_data::{CompilerConfig, ConstructorState},
    error::{ErrorCollector, ErrorKind, ErrorRecord},
    global_decl_collection::GlobalDeclCollector,
    kasl_parser,
    scope_graph_analyzing::ScopeGraphAnalyzer,
    scope_manager::{IOBlueprint, ScopeGraph},
    statement_building::StatementBuilder,
    struct_graph_analyzing::StructGraphAnalyzer,
};
use std::mem;

#[derive(Default)]
pub struct TestContext {
    pub ec: ErrorCollector,
    pub namespace: NameSpace,
    pub comp_data: CompilationData,
    pub scope_graph: ScopeGraph,
    pub builtin_registry: BuiltinRegistry,

    pub comp_config: CompilerConfig,
    pub constructor_state: ConstructorState,
}

pub fn parse_expr(input: &str) -> Vec<ParserDeclStmt> {
    kasl_parser::parse(input).unwrap()
}

pub fn collect_global_decls(
    test_ctx: &mut TestContext,
    statements: &[ParserDeclStmt],
) -> Result<(), Vec<ErrorRecord>> {
    let mut global_decl_collector = GlobalDeclCollector::new(
        &mut test_ctx.ec,
        &mut test_ctx.namespace,
        &mut test_ctx.comp_data,
        &test_ctx.comp_config,
        &test_ctx.builtin_registry,
        &mut test_ctx.scope_graph,
        &test_ctx.constructor_state,
    );
    global_decl_collector.process(statements);
    test_ctx.ec.as_result()
}

pub fn analyze_structs(test_ctx: &mut TestContext) -> Result<(), Vec<ErrorRecord>> {
    let mut struct_graph_analyzer = StructGraphAnalyzer::new(
        &mut test_ctx.ec,
        &test_ctx.namespace,
        &test_ctx.comp_data.struct_graph,
    );
    struct_graph_analyzer.analyze_all();
    test_ctx.ec.as_result()
}

pub fn build_stmts(test_ctx: &mut TestContext) -> Result<(), Vec<ErrorRecord>> {
    let mut stmt_builder = StatementBuilder::new(
        &mut test_ctx.ec,
        &mut test_ctx.namespace,
        &test_ctx.comp_data,
        &test_ctx.builtin_registry,
        &mut test_ctx.scope_graph,
    );
    stmt_builder.build_all();
    test_ctx.ec.as_result()
}

pub fn analyze_scopes(test_ctx: &mut TestContext) -> Result<(), Vec<ErrorRecord>> {
    let mut scope_graph_analyzer = ScopeGraphAnalyzer::new(
        &mut test_ctx.ec,
        &test_ctx.namespace,
        &mut test_ctx.scope_graph,
    );
    scope_graph_analyzer.analyze_all();
    test_ctx.ec.as_result()
}

pub fn build_blueprint(test_ctx: &mut TestContext) -> IOBlueprint {
    let blueprint_builder = BlueprintBuilder::new(&test_ctx.namespace);
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
    let main_func_id = test_ctx
        .namespace
        .func_ctx
        .get_global_func_by_name("main")
        .unwrap();
    let code = backend
        .compile(
            &test_ctx.namespace,
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

pub fn assert_error(error: &[ErrorRecord], expected: ErrorKind) {
    assert!(error.iter().any(|r| r.key.kind == expected))
}
