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

mod recursive_resolver;

use crate::{
    ast_nodes::{CompilationData, NameSpaceID, ScopeID, compilation_data::ProgramContext},
    builtin::BuiltinRegistry,
    error::ErrorCollector,
};

/// Resolves the L-value expressions used in assign statement from the list of parsed expression tokens.
pub struct LValueResolver<'a> {
    ec: &'a mut ErrorCollector,
    prog_ctx: &'a mut ProgramContext,
    comp_data: &'a mut CompilationData,
    builtin_registry: &'a BuiltinRegistry,
    current_scope: ScopeID,
    current_namespace: NameSpaceID,
}

impl<'a> LValueResolver<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        prog_ctx: &'a mut ProgramContext,
        comp_data: &'a mut CompilationData,
        builtin_registry: &'a BuiltinRegistry,
        current_scope: ScopeID,
        current_namespace: NameSpaceID,
    ) -> Self {
        Self {
            ec,
            prog_ctx,
            comp_data,
            builtin_registry,
            current_scope,
            current_namespace,
        }
    }
}
