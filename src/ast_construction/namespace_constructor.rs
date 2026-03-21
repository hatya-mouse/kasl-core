use crate::{
    CompilationData, NameSpaceID, ParserDeclStmt,
    builtin::BuiltinRegistry,
    compilation_data::{CompilerState, ProgramContext},
    error::ErrorCollector,
    global_decl_collection::GlobalDeclCollector,
    kasl_parser,
};
use peg::{error::ParseError, str::LineCol};

/// Constructs a single namespace from a raw source code string.
pub struct NameSpaceConstructor<'a> {
    ec: &'a mut ErrorCollector,
    prog_ctx: &'a mut ProgramContext,
    comp_data: &'a mut CompilationData,
    comp_state: CompilerState,
    builtin_registry: &'a BuiltinRegistry,

    code: String,
    decl_stmts: Vec<ParserDeclStmt>,
    namespace_id: NameSpaceID,
}

impl<'a> NameSpaceConstructor<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        prog_ctx: &'a mut ProgramContext,
        comp_data: &'a mut CompilationData,
        comp_state: CompilerState,
        builtin_registry: &'a BuiltinRegistry,
        namespace_id: NameSpaceID,
    ) -> Self {
        Self {
            ec,
            prog_ctx,
            comp_data,
            comp_state,
            builtin_registry,
            code: String::new(),
            decl_stmts: Vec::new(),
            namespace_id,
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
            self.ec,
            self.prog_ctx,
            self.comp_data,
            &self.comp_state,
            self.builtin_registry,
            self.namespace_id,
        );
        global_decl_collector.process(&self.decl_stmts);
    }
}
