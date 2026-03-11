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

use crate::{FunctionID, backend::func_translator::FuncTranslator};
use cranelift_codegen::ir;

impl<'a> FuncTranslator<'a> {
    pub fn translate_func_call_expr(&mut self) -> ir::Value {}

    pub fn call_func(&'a mut self, func_id: &FunctionID) -> ir::Value {
        // Create a return block
        let return_block = self.builder.create_block();
        // Create a translator
        let mut translator = FuncTranslator::new(
            &mut self.builder,
            self.module,
            self.comp_state,
            return_block,
        );
        translator.translate(func_id);

        // Add some arguments to the return block
        translator.builder.switch_to_block(return_block);
        translator.builder.seal_block(return_block);

        // Retrieve the return value
        translator.builder.block_params(return_block)[0]
    }
}
