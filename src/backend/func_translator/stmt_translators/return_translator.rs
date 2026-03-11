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

use crate::{Expr, backend::func_translator::FuncTranslator, type_registry::ResolvedType};
use cranelift::prelude::InstBuilder;
use cranelift_codegen::ir::{self, BlockArg};

impl FuncTranslator<'_> {
    pub fn translate_return(
        &mut self,
        value: &Option<Expr<ResolvedType>>,
        return_block: ir::Block,
    ) {
        if let Some(return_val) = value.as_ref().map(|val| self.translate_expr(val)) {
            self.builder
                .ins()
                .jump(return_block, &[BlockArg::Value(return_val)]);
        } else {
            self.builder.ins().jump(return_block, &[]);
        }
    }
}
