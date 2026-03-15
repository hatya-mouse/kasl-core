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
    CompilationState, ParserDeclStmt, ProgramContext,
    blueprint_builder::BlueprintBuilder,
    builtin::BuiltinRegistry,
    compilation_data::CompilerConfig,
    error::ErrorCollector,
    global_decl_collection::GlobalDeclCollector,
    kasl_parser,
    scope_graph_analyzing::ScopeGraphAnalyzer,
    scope_manager::{IOBlueprint, ScopeGraph},
    statement_building::StatementBuilder,
    struct_graph_analyzing::StructGraphAnalyzer,
};
use peg::{error::ParseError, str::LineCol};
use std::{collections::HashSet, path::PathBuf};

#[derive(Default)]
pub struct ProgramConstructor {
    ec: ErrorCollector,
    pub prog_ctx: ProgramContext,
    comp_state: CompilationState,
    comp_config: CompilerConfig,
    builtin_registry: BuiltinRegistry,
    scope_graph: ScopeGraph,

    code: String,
    decl_stmts: Vec<ParserDeclStmt>,

    imported_paths: HashSet<PathBuf>,
}

impl ProgramConstructor {
    pub fn add_search_paths(&mut self, paths: &[PathBuf]) {
        for path in paths {
            self.comp_config.search_paths.push(path.clone());
        }
    }

    pub fn set_imported_paths(&mut self, paths: &HashSet<PathBuf>) {
        self.imported_paths = paths.clone();
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
            &mut self.prog_ctx,
            &mut self.comp_state,
            &self.comp_config,
            &self.builtin_registry,
            &mut self.scope_graph,
            &mut self.imported_paths,
        );
        global_decl_collector.process(&self.decl_stmts);
    }

    pub fn analyze_struct_graph(&mut self) {
        let mut struct_graph_analyzer =
            StructGraphAnalyzer::new(&mut self.ec, &self.prog_ctx, &self.comp_state.struct_graph);
        struct_graph_analyzer.analyze_all();
    }

    pub fn build_statements(&mut self) {
        let mut stmt_builder = StatementBuilder::new(
            &mut self.ec,
            &mut self.prog_ctx,
            &self.comp_state,
            &self.builtin_registry,
            &mut self.scope_graph,
        );
        stmt_builder.build_all();
    }

    pub fn analyze_scope_graph(&mut self) {
        let mut scope_graph_analyzer =
            ScopeGraphAnalyzer::new(&mut self.ec, &self.prog_ctx, &mut self.scope_graph);
        scope_graph_analyzer.analyze_all();
    }

    pub fn get_blueprint(&self) -> IOBlueprint {
        let blueprint_builder = BlueprintBuilder::new(&self.prog_ctx);
        blueprint_builder.build()
    }
}
