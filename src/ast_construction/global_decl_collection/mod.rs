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

mod resolvers;
mod stmt_process;

use std::{collections::HashSet, path::PathBuf};

pub use resolvers::FuncDeclInfo;

use crate::{
    CompilationState, ParserDeclStmt, ProgramContext, builtin::BuiltinRegistry,
    compilation_data::CompilerConfig, error::ErrorCollector, scope_manager::ScopeGraph,
};

pub struct GlobalDeclCollector<'a> {
    ec: &'a mut ErrorCollector,
    prog_ctx: &'a mut ProgramContext,
    comp_state: &'a mut CompilationState,
    comp_config: &'a CompilerConfig,
    builtin_registry: &'a BuiltinRegistry,
    scope_graph: &'a mut ScopeGraph,

    imported_paths: &'a mut HashSet<PathBuf>,
}

impl<'a> GlobalDeclCollector<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        prog_ctx: &'a mut ProgramContext,
        comp_state: &'a mut CompilationState,
        comp_config: &'a CompilerConfig,
        builtin_registry: &'a BuiltinRegistry,
        scope_graph: &'a mut ScopeGraph,
        imported_paths: &'a mut HashSet<PathBuf>,
    ) -> Self {
        Self {
            ec,
            prog_ctx,
            comp_state,
            comp_config,
            builtin_registry,
            scope_graph,
            imported_paths,
        }
    }

    pub fn process(&mut self, decl_stmts: &'a [ParserDeclStmt]) {
        for stmt in decl_stmts.iter() {
            self.process_stmt(stmt);
        }
    }
}
