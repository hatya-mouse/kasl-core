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

use crate::{CompilationState, FunctionID, backend::func_translator::FuncTranslator};
use cranelift::prelude::{Configurable, FunctionBuilder, FunctionBuilderContext};
use cranelift_codegen::settings;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::Module;

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
    pub fn translate(&mut self, comp_state: &CompilationState, entry_point: &FunctionID) {
        // Create a function builder
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_ctx);
        // Create an entry block and and switch to the block
        let entry_block = builder.create_block();
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        // Create a FuncTranslator and translate the function
        let mut translator = FuncTranslator::new(builder, &self.module, comp_state);
        translator.translate(entry_point);

        // Finalize the function
        translator.builder.finalize();
    }
}
