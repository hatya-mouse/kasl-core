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

mod block_translator;
mod expr_translators;
mod io_blueprint;
mod stmt_translators;
mod translator_scope_registry;
mod type_converter;

use cranelift_codegen::ir;
pub use type_converter::TypeConverter;

use crate::{
    FunctionID, backend::func_translator::translator_scope_registry::TranslatorScopeRegistry,
    builtin::BuiltinRegistry, compilation_data::ProgramContext, scope_manager::IOBlueprint,
};
use cranelift::prelude::FunctionBuilder;
use cranelift_jit::JITModule;

pub struct FuncTranslator<'a> {
    pub builder: FunctionBuilder<'a>,
    type_converter: TypeConverter,

    prog_ctx: &'a ProgramContext,
    builtin_registry: &'a BuiltinRegistry,
    scope_registry: TranslatorScopeRegistry,
}

impl<'a> FuncTranslator<'a> {
    pub fn new(
        builder: FunctionBuilder<'a>,
        module: &'a JITModule,
        prog_ctx: &'a ProgramContext,
        builtin_registry: &'a BuiltinRegistry,
    ) -> Self {
        let type_converter = TypeConverter::new(module);
        let mut scope_registry = TranslatorScopeRegistry::default();
        // Add the global scope registry
        scope_registry.push_deepest();

        Self {
            builder,
            type_converter,
            prog_ctx,
            builtin_registry,
            scope_registry,
        }
    }

    pub fn translate(
        &mut self,
        entry_point: &FunctionID,
        blueprint: &IOBlueprint,
        entry_block: ir::Block,
        return_block: ir::Block,
    ) {
        // Get the pointer to the pointer array
        let block_params = self.builder.block_params(entry_block);
        let input_ptr_ptr = block_params[0];
        let output_ptr_ptr = block_params[1];
        let state_ptr_ptr = block_params[2];
        let should_init = block_params[3];

        // Get the input and state variables from the blueprint
        self.load_blueprint_access(input_ptr_ptr, state_ptr_ptr, should_init, blueprint);

        // Get the entry point function node
        let Some(func_block) = self
            .prog_ctx
            .func_ctx
            .get_func(entry_point)
            .map(|func| &func.block)
        else {
            return;
        };

        self.translate_block(func_block, return_block);

        // Push the values in the states and outputs to the original pointer
        self.store_blueprint(output_ptr_ptr, state_ptr_ptr, blueprint);
    }
}
