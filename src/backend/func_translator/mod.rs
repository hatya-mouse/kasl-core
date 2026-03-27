mod block_translator;
mod expr_translators;
mod io_blueprint;
mod stmt_translators;
mod translator_scope_registry;
mod type_converter;
mod utils;

pub use type_converter::TypeConverter;

use crate::{
    FunctionID, backend::func_translator::translator_scope_registry::TranslatorScopeRegistry,
    builtin::BuiltinRegistry, compilation_data::ProgramContext, scope_manager::IOBlueprint,
};
use cranelift::prelude::FunctionBuilder;
use cranelift_codegen::ir;
use cranelift_jit::JITModule;

pub struct FuncTranslator<'a> {
    pub builder: FunctionBuilder<'a>,
    module: &'a mut JITModule,
    type_converter: TypeConverter,

    prog_ctx: &'a ProgramContext,
    builtin_registry: &'a BuiltinRegistry,
    scope_registry: TranslatorScopeRegistry,
}

impl<'a> FuncTranslator<'a> {
    pub fn new(
        builder: FunctionBuilder<'a>,
        module: &'a mut JITModule,
        prog_ctx: &'a ProgramContext,
        builtin_registry: &'a BuiltinRegistry,
    ) -> Self {
        let type_converter = TypeConverter::new(module);
        let mut scope_registry = TranslatorScopeRegistry::default();
        // Add the global scope registry
        scope_registry.push_deepest();

        Self {
            builder,
            module,
            type_converter,
            prog_ctx,
            builtin_registry,
            scope_registry,
        }
    }

    pub fn translate(
        &mut self,
        params: TranslatorParams,
        iteration_index: Option<ir::Value>,
        entry_point: &FunctionID,
        blueprint: &IOBlueprint,
        exit_block: ir::Block,
    ) {
        // Get the input, output, state variables, and constants from the blueprint
        self.load_blueprint_access(&params, blueprint, iteration_index);

        // Get the entry point function node
        let Some(func_block) = self
            .prog_ctx
            .func_ctx
            .get_func(entry_point)
            .map(|func| &func.block)
        else {
            return;
        };

        self.translate_block(func_block, exit_block);

        // Push the values in the states and outputs to the original pointer
        self.store_blueprint(&params, blueprint, iteration_index);
    }
}

pub struct TranslatorParams {
    pub input_ptr_ptr: ir::Value,
    pub output_ptr_ptr: ir::Value,
    pub state_ptr_ptr: ir::Value,
    pub should_init: ir::Value,
}
