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
    CompilationData, ParserDeclStmt, Range,
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
use peg::{error::ParseError, str::LineCol};
use std::{mem, path::PathBuf};

#[derive(Default)]
pub struct KaslCompiler {
    ec: ErrorCollector,
    pub prog_ctx: ProgramContext,
    comp_data: CompilationData,
    comp_state: CompilerState,
    builtin_registry: BuiltinRegistry,

    parser_decl_stmts: Vec<ParserDeclStmt>,
    compiled: *const u8,
}

impl KaslCompiler {
    pub fn add_search_path(&mut self, path: PathBuf) {
        self.comp_state.child_search_paths.push(path);
    }

    pub fn parse(&mut self, code: &str) -> Result<(), ParseError<LineCol>> {
        self.parser_decl_stmts = kasl_parser::parse(code)?;
        println!("{:#?}", self.parser_decl_stmts);
        Ok(())
    }

    pub fn build(&mut self) -> Result<IOBlueprint, Vec<ErrorRecord>> {
        // 1. Collect global declarations
        let root_namespace = self.prog_ctx.namespace_registry.get_root_namespace_id();
        let mut global_decl_collector = GlobalDeclCollector::new(
            &mut self.ec,
            &mut self.prog_ctx,
            &mut self.comp_data,
            &self.comp_state,
            &self.builtin_registry,
            root_namespace,
        );
        global_decl_collector.process(&self.parser_decl_stmts);

        // 2. Analyze struct graph
        let mut struct_analyzer =
            StructGraphAnalyzer::new(&mut self.ec, &self.prog_ctx, &self.comp_data.struct_graph);
        struct_analyzer.analyze_all();

        // 3. Build statements
        let mut stmt_builder = StatementBuilder::new(
            &mut self.ec,
            &mut self.prog_ctx,
            &mut self.comp_data,
            &self.builtin_registry,
        );
        stmt_builder.build_all();

        // 4. Analyze scope graph
        let mut scope_analyzer = ScopeGraphAnalyzer::new(
            &mut self.ec,
            &self.prog_ctx,
            &mut self.comp_data.scope_graph,
        );
        scope_analyzer.analyze_all();

        // 5. Build an IOBlueprint
        let blueprint_builder = BlueprintBuilder::new(&self.prog_ctx);
        let blueprint = blueprint_builder.build();

        self.ec.as_result().map(|_| blueprint)
    }

    pub fn compile(&mut self, blueprint: &IOBlueprint) -> Result<(), Vec<ErrorRecord>> {
        // 6. Compile the program
        let mut backend = Backend::default();
        let root_namespace_id = self.prog_ctx.namespace_registry.get_root_namespace_id();
        let main_func_id = self
            .prog_ctx
            .func_ctx
            .get_global_func_id(root_namespace_id, "main")
            .unwrap();
        self.compiled = backend
            .compile(
                &self.prog_ctx,
                &self.builtin_registry,
                blueprint,
                &main_func_id,
            )
            .map_err(|e| {
                vec![ErrorRecord::new(
                    ErrorKey::new(EK::CompilerBug, Pl::Str(e)),
                    Range::zero(),
                    Ph::Backend,
                    Sv::Error,
                )]
            })?;

        Ok(())
    }

    pub fn run(
        &mut self,
        inputs: &[*mut ()],
        outputs: &[*mut ()],
        states: &[*mut ()],
        should_init: i8,
    ) -> Result<(), String> {
        unsafe {
            KaslCompiler::run_code(
                self.compiled,
                inputs.as_ptr(),
                outputs.as_ptr(),
                states.as_ptr(),
                should_init,
            );
        }

        Ok(())
    }

    unsafe fn run_code(
        code_ptr: *const u8,
        input: *const *mut (),
        output: *const *mut (),
        state: *const *mut (),
        should_init: i8,
    ) {
        unsafe {
            let code_fn: fn(*const *mut (), *const *mut (), *const *mut (), i8) =
                mem::transmute(code_ptr);
            code_fn(input, output, state, should_init)
        }
    }
}
