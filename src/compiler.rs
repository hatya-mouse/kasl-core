//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

//! The orchestrator of the entire compilation process, from parsing to code generation.

use crate::{
    MAIN_FUNCTION_NAME,
    ast::{
        CompilationData, Range,
        compilation_data::{CompilerState, ProgramContext},
        scope_manager::IOBlueprint,
    },
    ast_construction::{
        BlueprintBuilder, GlobalDeclCollector, ScopeGraphAnalyzer, StatementBuilder,
        flow_graph_analyzing::FlowGraphAnalyzer,
    },
    builtin::BuiltinRegistry,
    error::{EK, ErrorCollector, ErrorKey, ErrorRecord, Ph, Pl, Sv},
    lowerer::Lowerer,
    parser::{ParserDeclStmt, kasl_parser},
};
use kasl_ir::Function;
use std::path::PathBuf;

/// The main compiler struct that manages the entire compilation process, from parsing to code generation.
///
/// If you want to run the process individually, you might want to use the individual components directly,
/// because this struct provides a convenient interface for the entire process.
///
/// # How does it work?
///
/// The compiler processes the KASL code in several stages:
/// 1. **Parsing**: The `parse` method takes the KASL code.
/// 2. **Building**: The `build` method analyzes the parsed code and constructs an `IOBlueprint`. Building phase has several sub-stages:
///     1. Collect global declarations (e.g. `input`) using `GlobalDeclCollector`.
///     2. Analyze struct graph and find recursive structs using `StructGraphAnalyzer`.
///     3. Build statements inside functions using `StatementBuilder`.
///     4. Analyze scope graph and find recursive calls using `ScopeGraphAnalyzer`.
///     5. Finally, build an `IOBlueprint` using `BlueprintBuilder`.
/// 3. **Lowering**: Translate the AST into KASL-IR, an intermediate expression for KASL.
///
/// # Usage
/// ```rust
/// use kasl_core::{KaslCompiler, run_once};
///
/// let code = r#"
/// output out_value = 0
///
/// func main() {
///     out_value = Builtin.iadd(1, 1)
/// }
/// "#;
///
/// // Create a new instance of the compiler
/// let mut compiler = KaslCompiler::default();
///
/// // Parse the KASL code
/// compiler.parse(code).expect("Failed to parse code");
///
/// // Analyze and build the program, which returns an IOBlueprint
/// let blueprint = compiler.build().expect("Failed to build program");
///
/// // Allocate a memory for the output value based on the blueprint
/// let out_value_size = blueprint.get_outputs()[0].actual_size;
/// let out_value_ptr = unsafe {
///     let layout = std::alloc::Layout::from_size_align(out_value_size as usize, 1).unwrap();
///     std::alloc::alloc(layout) as *mut ()
/// };
///
/// // Lower the program to KASL-IR
/// let func = compiler.lower_once(&blueprint).expect("Failed to compile program");
/// ```
#[derive(Default)]
pub struct KaslCompiler {
    ec: ErrorCollector,
    /// The current program context, which mainly holds the constructed AST and related information.
    prog_ctx: ProgramContext,
    comp_state: CompilerState,

    /// The raw declarations parsed from the KASL code.
    parser_decl_stmts: Vec<ParserDeclStmt>,
}

impl KaslCompiler {
    /// Adds a search path for the compiler to look for imported modules.
    pub fn add_search_path(&mut self, path: PathBuf) {
        self.comp_state.child_search_paths.push(path);
    }

    /// Sets the search paths for the compiler to look for imported modules, replacing any existing paths.
    pub fn set_search_paths(&mut self, paths: Vec<PathBuf>) {
        self.comp_state.child_search_paths = paths;
    }

    /// Clears all search paths for the compiler.
    pub fn clear_search_paths(&mut self) {
        self.comp_state.child_search_paths.clear();
    }

    /// Adds a virtual file to the compiler state.
    pub fn add_virtual_file(&mut self, path: PathBuf, content: String) {
        self.comp_state.virtual_files.insert(path, content);
    }

    /// Returns a reference to the program context in the compiler.
    pub fn get_prog_ctx(&self) -> &ProgramContext {
        &self.prog_ctx
    }

    /// Parses the given KASL code and stores the resulting declarations in the compiler's state.
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

    /// Builds the program from the stored declarations, and returns an `IOBlueprint` that represents the program's inputs outputs and states structure.
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

        // 2. Build statements
        let mut stmt_builder = StatementBuilder::new(
            &mut self.ec,
            &mut self.prog_ctx,
            &mut comp_data,
            &builtin_registry,
        );
        stmt_builder.build_all();

        // 3. Analyze flow graph
        let mut flow_analyzer = FlowGraphAnalyzer::new(
            &mut self.ec,
            &self.prog_ctx,
            &comp_data.func_flow_graphs,
            &comp_data.op_flow_graphs,
        );
        flow_analyzer.analyze_all();

        println!(
            "is_prime id: {:#?}",
            self.prog_ctx.func_ctx.get_global_func_id(
                self.prog_ctx.namespace_registry.get_root_namespace_id(),
                "is_prime"
            )
        );
        println!("{:#?}", comp_data.func_flow_graphs);

        // 4. Analyze scope graph
        let mut scope_analyzer =
            ScopeGraphAnalyzer::new(&mut self.ec, &self.prog_ctx, &mut comp_data.scope_graph);
        scope_analyzer.analyze_all();

        // 5. Build an IOBlueprint
        let blueprint_builder = BlueprintBuilder::new(&self.prog_ctx);
        let blueprint = blueprint_builder.build();

        self.ec.as_result().map(|_| blueprint)
    }

    /// Translates the program into KASL-IR, an intermediate representation for KASL language.
    /// The lowered program will run only once.
    pub fn lower_once(&mut self, blueprint: &IOBlueprint) -> Result<Function, ErrorRecord> {
        let builtin_registry = BuiltinRegistry::default();

        // Create a lowerer and lower the program
        let lowerer = Lowerer::default();

        // Get the main function
        let root_namespace_id = self.prog_ctx.namespace_registry.get_root_namespace_id();
        // Look up the main function, or return an error if it doesn't exist
        let main_func_id = self
            .prog_ctx
            .func_ctx
            .get_global_func_id(root_namespace_id, MAIN_FUNCTION_NAME)
            .ok_or(ErrorRecord::new(
                ErrorKey::new(EK::NoMainFunc, Pl::None),
                Range::zero(),
                Ph::Backend,
                Sv::Error,
            ))?;

        // Lower the program
        let func = lowerer.lower_once(&self.prog_ctx, &builtin_registry, blueprint, &main_func_id);

        Ok(func)
    }

    /// Translates the program into KASL-IR, an intermediate representation for KASL language.
    /// The lowered program will run specified times. Iterations can be given by the fifth parameter of the compiled function.
    pub fn lower_buffer(&mut self, blueprint: &IOBlueprint) -> Result<Function, ErrorRecord> {
        let builtin_registry = BuiltinRegistry::default();

        // Create a lowerer and lower the program
        let lowerer = Lowerer::default();

        // Get the main function
        let root_namespace_id = self.prog_ctx.namespace_registry.get_root_namespace_id();
        // Look up the main function, or return an error if it doesn't exist
        let main_func_id = self
            .prog_ctx
            .func_ctx
            .get_global_func_id(root_namespace_id, MAIN_FUNCTION_NAME)
            .ok_or(ErrorRecord::new(
                ErrorKey::new(EK::NoMainFunc, Pl::None),
                Range::zero(),
                Ph::Backend,
                Sv::Error,
            ))?;

        // Lower the program
        let func =
            lowerer.lower_buffer(&self.prog_ctx, &builtin_registry, blueprint, &main_func_id);

        Ok(func)
    }
}
