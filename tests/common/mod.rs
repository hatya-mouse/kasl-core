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
    CompilationState, NameSpace, ParserDeclStmt,
    error::{ErrorCollector, ErrorKind, ErrorRecord},
    global_decl_collection::GlobalDeclCollector,
    kasl_parser,
    scope_manager::ScopeGraph,
    statement_building::StatementBuilder,
    struct_graph_analyzing::StructGraphAnalyzer,
    symbol_table::{FuncBodyMap, OpBodyMap},
    type_registry::StructGraph,
};

#[derive(Default)]
pub struct TestContext {
    pub ec: ErrorCollector,
    pub name_space: NameSpace,
    pub func_body_map: FuncBodyMap,
    pub op_body_map: OpBodyMap,
    pub comp_state: CompilationState,
    pub scope_graph: ScopeGraph,
    pub struct_graph: StructGraph,
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
        &mut test_ctx.name_space,
        &mut test_ctx.func_body_map,
        &mut test_ctx.op_body_map,
        &mut test_ctx.comp_state,
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
    println!("{:#?}", struct_graph_analyzer);
    test_ctx.ec.as_result()
}

pub fn build_stmts(test_ctx: &mut TestContext) -> Result<(), Vec<ErrorRecord>> {
    let mut stmt_builder = StatementBuilder::new(
        &mut test_ctx.ec,
        &mut test_ctx.name_space,
        &test_ctx.func_body_map,
        &test_ctx.op_body_map,
        &mut test_ctx.comp_state,
        &mut test_ctx.scope_graph,
    );
    stmt_builder.build_all();
    test_ctx.ec.as_result()
}

pub fn assert_error(error: &[ErrorRecord], expected: ErrorKind) {
    assert!(error.iter().any(|r| r.key.kind == expected))
}
