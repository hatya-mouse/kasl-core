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

mod func_translator;

use crate::{
    FunctionID, backend::func_translator::FuncTranslator, builtin::BuiltinRegistry,
    compilation_data::ProgramContext, scope_manager::IOBlueprint,
};
use cranelift::prelude::{
    AbiParam, Configurable, FunctionBuilder, FunctionBuilderContext, InstBuilder,
};
use cranelift_codegen::{settings, verify_function};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};

pub struct Backend {
    builder_ctx: FunctionBuilderContext,
    ctx: cranelift_codegen::Context,
    module: JITModule,
}

impl Default for Backend {
    fn default() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        flag_builder.set("opt_level", "speed").unwrap();
        flag_builder.set("enable_alias_analysis", "true").unwrap();
        let isa_builder = cranelift_native::builder()
            .unwrap_or_else(|msg| panic!("The host machine is not supported: {}", msg));
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();
        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        let module = JITModule::new(builder);

        Self {
            builder_ctx: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            module,
        }
    }
}

impl Backend {
    pub fn compile(
        &mut self,
        prog_ctx: &ProgramContext,
        builtin_registry: &BuiltinRegistry,
        blueprint: &IOBlueprint,
        entry_point: &FunctionID,
    ) -> Result<*const u8, String> {
        self.translate(prog_ctx, builtin_registry, blueprint, entry_point);

        // Verify the function
        let verifier_flags = settings::Flags::new(settings::builder());
        verify_function(&self.ctx.func, &verifier_flags).map_err(|e| e.to_string())?;

        let id = self
            .module
            .declare_function("main", Linkage::Export, &self.ctx.func.signature)
            .map_err(|e| e.to_string())?;
        self.module
            .define_function(id, &mut self.ctx)
            .map_err(|e| e.to_string())?;

        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions().unwrap();

        let code = self.module.get_finalized_function(id);
        Ok(code)
    }

    pub fn translate(
        &mut self,
        prog_ctx: &ProgramContext,
        builtin_registry: &BuiltinRegistry,
        blueprint: &IOBlueprint,
        entry_point: &FunctionID,
    ) {
        // Add parameter for the input and output pointers
        let pointer_type = self.module.target_config().pointer_type();
        self.ctx.func.signature.params.extend(&[
            AbiParam::new(pointer_type),
            AbiParam::new(pointer_type),
            AbiParam::new(pointer_type),
        ]);

        // Create a function builder
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_ctx);

        // Create an entry block and and switch to the block
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);

        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        // Create a return block
        let return_block = builder.create_block();

        // Create a FuncTranslator and translate the function
        let mut translator = FuncTranslator::new(builder, &self.module, prog_ctx, builtin_registry);
        translator.translate(entry_point, blueprint, entry_block, return_block);

        // Add jump instruction
        translator.builder.ins().jump(return_block, &[]);

        // Add return instruction to the return block
        translator.builder.switch_to_block(return_block);
        translator.builder.seal_block(return_block);

        translator.builder.ins().return_(&[]);

        // Finalize the function
        translator.builder.finalize();
    }
}
