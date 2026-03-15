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

use std::{collections::HashSet, path::PathBuf};

use crate::{
    CompilationData, NameSpace, ParserDeclStmt,
    blueprint_builder::BlueprintBuilder,
    builtin::BuiltinRegistry,
    compilation_data::{CompilerConfig, ConstructorState},
    error::ErrorCollector,
    global_decl_collection::GlobalDeclCollector,
    kasl_parser,
    scope_graph_analyzing::ScopeGraphAnalyzer,
    scope_manager::{IOBlueprint, ScopeGraph},
    statement_building::StatementBuilder,
    struct_graph_analyzing::StructGraphAnalyzer,
};
use peg::{error::ParseError, str::LineCol};

pub struct NameSpaceConstructor {
    ec: ErrorCollector,
    pub namespace: NameSpace,
    comp_data: CompilationData,
    comp_config: CompilerConfig,
    builtin_registry: BuiltinRegistry,
    scope_graph: ScopeGraph,

    constructor_state: ConstructorState,

    code: String,
    decl_stmts: Vec<ParserDeclStmt>,
}

impl NameSpaceConstructor {
    pub fn new(comp_config: CompilerConfig, imported_paths: HashSet<PathBuf>) -> Self {
        let constructor_state = ConstructorState { imported_paths };

        Self {
            ec: ErrorCollector::default(),
            namespace: NameSpace::default(),
            comp_data: CompilationData::default(),
            comp_config,
            builtin_registry: BuiltinRegistry::default(),
            scope_graph: ScopeGraph::default(),
            constructor_state,
            code: String::new(),
            decl_stmts: Vec::new(),
        }
    }

    pub fn set_code(&mut self, code: &str) {
        self.code = code.to_string();
    }

    // --- PROCESS FUNCTIONS ---

    pub fn parse(&mut self) -> Result<(), ParseError<LineCol>> {
        self.decl_stmts = kasl_parser::parse(&self.code)?;
        Ok(())
    }

    pub fn collect_global_decls(&mut self) {
        let mut global_decl_collector = GlobalDeclCollector::new(
            &mut self.ec,
            &mut self.namespace,
            &mut self.comp_data,
            &self.comp_config,
            &self.builtin_registry,
            &mut self.scope_graph,
            &self.constructor_state,
        );
        global_decl_collector.process(&self.decl_stmts);
    }

    pub fn analyze_struct_graph(&mut self) {
        let mut struct_graph_analyzer =
            StructGraphAnalyzer::new(&mut self.ec, &self.namespace, &self.comp_data.struct_graph);
        struct_graph_analyzer.analyze_all();
    }

    pub fn build_statements(&mut self) {
        let mut stmt_builder = StatementBuilder::new(
            &mut self.ec,
            &mut self.namespace,
            &self.comp_data,
            &self.builtin_registry,
            &mut self.scope_graph,
        );
        stmt_builder.build_all();
    }

    pub fn analyze_scope_graph(&mut self) {
        let mut scope_graph_analyzer =
            ScopeGraphAnalyzer::new(&mut self.ec, &self.namespace, &mut self.scope_graph);
        scope_graph_analyzer.analyze_all();
    }

    pub fn get_blueprint(&self) -> IOBlueprint {
        let blueprint_builder = BlueprintBuilder::new(&self.namespace);
        blueprint_builder.build()
    }
}
