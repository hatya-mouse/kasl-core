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

mod blueprint;
mod expr_translator;
mod ir_scope_registry;
mod stmt_translator;
mod type_converter;
mod utils;

use crate::{
    ast::{FunctionID, compilation_data::ProgramContext, scope_manager::IOBlueprint},
    lowerer::TranslatorParams,
};
use ir_scope_registry::IRScopeRegistry;
use kasl_ir::ir::{Block, IRBuilder, Value};

pub struct FuncTranslator<'a> {
    /// IR builder used to build the function body.
    pub builder: IRBuilder,
    /// Program context to be translated.
    prog_ctx: &'a ProgramContext,
    /// Scope registry to manage the variables declared while translating the program context.
    scope_registry: IRScopeRegistry,
}

impl<'a> FuncTranslator<'a> {
    pub(super) fn new(builder: IRBuilder, prog_ctx: &'a ProgramContext) -> Self {
        let mut scope_registry = IRScopeRegistry::default();
        // Add the global scope registry
        scope_registry.push_deepest();

        Self {
            builder,
            prog_ctx,
            scope_registry,
        }
    }

    pub(super) fn translate(
        &mut self,
        params: TranslatorParams,
        iteration_index: Option<Value>,
        entry_point: &FunctionID,
        blueprint: &IOBlueprint,
        exit_block: Block,
    ) {
        // Get the input, output, state variables, and constants from the blueprint
    }
}
