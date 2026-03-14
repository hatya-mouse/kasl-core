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

use crate::{
    CompilationState, NameSpace, ParserDeclStmt,
    error::{ErrorCollector, ErrorRecord},
    global_decl_collection::GlobalDeclCollector,
    scope_graph_analyzing::ScopeGraphAnalyzer,
    scope_manager::ScopeGraph,
    statement_building::StatementBuilder,
    struct_graph_analyzing::StructGraphAnalyzer,
    symbol_table::{FuncBodyMap, OpBodyMap},
    type_registry::StructGraph,
};

pub fn construct_program(statements: Vec<ParserDeclStmt>) -> Result<(), Vec<ErrorRecord>> {
    let mut ec = ErrorCollector::new();
    let mut name_space = NameSpace::default();
    let mut func_body_map = FuncBodyMap::default();
    let mut op_body_map = OpBodyMap::default();
    let mut comp_state = CompilationState::default();
    let mut scope_graph = ScopeGraph::default();
    let mut struct_graph = StructGraph::default();

    // 1. Collect global declarations, such as inputs, outputs, states, structs, struct fields and functions
    let mut global_decl_collector = GlobalDeclCollector::new(
        &mut ec,
        &mut name_space,
        &mut func_body_map,
        &mut op_body_map,
        &mut comp_state,
        &mut scope_graph,
        &mut struct_graph,
    );
    global_decl_collector.process(&statements);

    // 2. Analyze the struct graph
    let mut struct_graph_analyzer = StructGraphAnalyzer::new(&mut ec, &comp_state, &struct_graph);
    struct_graph_analyzer.analyze_all();

    // 3. Build the function bodies
    let mut stmt_builder = StatementBuilder::new(
        &mut ec,
        &mut name_space,
        &func_body_map,
        &op_body_map,
        &mut comp_state,
        &mut scope_graph,
    );
    stmt_builder.build_all();

    // 4. Analyze the scope graph
    let mut scope_graph_analyzer = ScopeGraphAnalyzer::new(&mut ec, &comp_state, &mut scope_graph);
    scope_graph_analyzer.analyze_all();

    ec.as_result()
}
