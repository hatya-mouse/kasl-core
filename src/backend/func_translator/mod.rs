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
mod stmt_translators;
mod type_converter;

pub use type_converter::TypeConverter;

use crate::{CompilationState, FunctionID, VariableID};
use cranelift::prelude::{FunctionBuilder, Variable};
use cranelift_jit::JITModule;
use std::collections::HashMap;

pub struct FuncTranslator<'a> {
    pub builder: FunctionBuilder<'a>,
    type_converter: TypeConverter,

    comp_state: &'a CompilationState,
    variables: HashMap<VariableID, Variable>,
}

impl<'a> FuncTranslator<'a> {
    pub fn new(
        builder: FunctionBuilder<'a>,
        module: &'a JITModule,
        comp_state: &'a CompilationState,
    ) -> Self {
        let type_converter = TypeConverter::new(module);

        Self {
            builder,
            type_converter,
            comp_state,
            variables: HashMap::new(),
        }
    }

    pub fn translate(&mut self, entry_point: &FunctionID) {
        // Get the entry point function node
        let Some(entry_func_node) = self.comp_state.func_ctx.get_func(entry_point) else {
            return;
        };

        if let Some(block) = &entry_func_node.block {
            self.translate_block(block);
        }
    }
}
