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

//! Global declaration collection phase of AST construction.

mod builders;
mod stmt_process;

pub use builders::FuncDeclInfo;

use crate::{
    ast_nodes::{
        CompilationData, NameSpaceID, ScopeID,
        compilation_data::{CompilerState, ProgramContext},
    },
    builtin::BuiltinRegistry,
    error::ErrorCollector,
    parser::ParserDeclStmt,
};

/// Collects the top level declarations such as `input`, `output`, `state`, `func`, etc.
pub struct GlobalDeclCollector<'a> {
    ec: &'a mut ErrorCollector,
    prog_ctx: &'a mut ProgramContext,
    comp_data: &'a mut CompilationData,
    comp_state: &'a CompilerState,
    builtin_registry: &'a BuiltinRegistry,

    current_namespace: NameSpaceID,
    current_scope_id: ScopeID,
}

impl<'a> GlobalDeclCollector<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        prog_ctx: &'a mut ProgramContext,
        comp_data: &'a mut CompilationData,
        comp_state: &'a CompilerState,
        builtin_registry: &'a BuiltinRegistry,
        current_namespace: NameSpaceID,
    ) -> Self {
        let global_scope_id = prog_ctx
            .scope_registry
            .get_global_scope_id(&current_namespace);

        // Set the current scope to the global scope of the current namespace
        Self {
            ec,
            prog_ctx,
            comp_data,
            comp_state,
            builtin_registry,
            current_namespace,
            current_scope_id: global_scope_id,
        }
    }

    /// Processes the given declaration statements.
    pub fn process(&mut self, decl_stmts: &'a [ParserDeclStmt]) {
        for stmt in decl_stmts.iter() {
            self.process_stmt(stmt);
        }
    }

    /// Marks the given name as used in the current scope.
    pub fn mark_name_used(&mut self, name: &str) {
        // Mark the name as used in the namespace
        self.prog_ctx
            .scope_registry
            .mark_name_used(&self.current_scope_id, name);
    }

    /// Returns `true` if the given name is used in the current scope.
    pub fn is_name_used(&self, name: &str) -> bool {
        self.prog_ctx
            .scope_registry
            .is_name_used(&self.current_scope_id, name)
    }

    /// Switches the current scope to the given scope ID.
    pub fn switch_to_scope(&mut self, scope_id: ScopeID) {
        self.current_scope_id = scope_id;
    }
}
