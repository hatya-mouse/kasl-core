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

use std::mem;

use kasl::{
    CompilationState, ParserDeclStmt,
    backend::Backend,
    builtin::BuiltinRegistry,
    error::{ErrorCollector, ErrorKind, ErrorRecord},
    global_decl_collection::GlobalDeclCollector,
    kasl_parser,
    scope_graph_analyzing::ScopeGraphAnalyzer,
    scope_manager::ScopeGraph,
    statement_building::StatementBuilder,
    struct_graph_analyzing::StructGraphAnalyzer,
    symbol_table::{FuncBodyMap, OpBodyMap},
    type_registry::StructGraph,
};

#[derive(Default)]
pub struct TestContext {
    pub ec: ErrorCollector,
    pub func_body_map: FuncBodyMap,
    pub op_body_map: OpBodyMap,
    pub comp_state: CompilationState,
    pub scope_graph: ScopeGraph,
    pub struct_graph: StructGraph,
    pub builtin_registry: BuiltinRegistry,
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
        &mut test_ctx.func_body_map,
        &mut test_ctx.op_body_map,
        &mut test_ctx.comp_state,
        &test_ctx.builtin_registry,
        &mut test_ctx.scope_graph,
        &mut test_ctx.struct_graph,
    );
    global_decl_collector.process(statements);
    test_ctx.ec.as_result()
}

pub fn analyze_structs(test_ctx: &mut TestContext) -> Result<(), Vec<ErrorRecord>> {
    let mut struct_graph_analyzer = StructGraphAnalyzer::new(
        &mut test_ctx.ec,
        &test_ctx.comp_state,
        &test_ctx.struct_graph,
    );
    struct_graph_analyzer.analyze_all();
    test_ctx.ec.as_result()
}

pub fn build_stmts(test_ctx: &mut TestContext) -> Result<(), Vec<ErrorRecord>> {
    let mut stmt_builder = StatementBuilder::new(
        &mut test_ctx.ec,
        &test_ctx.func_body_map,
        &test_ctx.op_body_map,
        &mut test_ctx.comp_state,
        &test_ctx.builtin_registry,
        &mut test_ctx.scope_graph,
    );
    stmt_builder.build_all();
    test_ctx.ec.as_result()
}

pub fn analyze_scopes(test_ctx: &mut TestContext) -> Result<(), Vec<ErrorRecord>> {
    let mut scope_graph_analyzer = ScopeGraphAnalyzer::new(
        &mut test_ctx.ec,
        &test_ctx.comp_state,
        &mut test_ctx.scope_graph,
    );
    scope_graph_analyzer.analyze_all();
    test_ctx.ec.as_result()
}

pub fn execute_program(test_ctx: &mut TestContext) -> i32 {
    let mut backend = Backend::default();
    let main_func_id = test_ctx
        .comp_state
        .func_ctx
        .get_global_func_by_name("main")
        .unwrap();
    let code = backend
        .compile(
            &test_ctx.comp_state,
            &test_ctx.builtin_registry,
            &main_func_id,
        )
        .unwrap();

    unsafe { run_code::<i32>(code) }
}

unsafe fn run_code<O>(code_ptr: *const u8) -> O {
    unsafe {
        let code_fn: fn() -> O = mem::transmute(code_ptr);
        code_fn()
    }
}

pub fn assert_error(error: &[ErrorRecord], expected: ErrorKind) {
    assert!(error.iter().any(|r| r.key.kind == expected))
}
