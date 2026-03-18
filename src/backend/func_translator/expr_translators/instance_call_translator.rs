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

use crate::{FunctionID, VariableID, backend::func_translator::FuncTranslator};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn translate_instance_call_expr(
        &mut self,
        id: &FunctionID,
        args: &[VariableID],
    ) -> ir::Value {
        // Call the function and get the result
        let func = self.prog_ctx.func_ctx.get_func(id).unwrap();
        self.call_func(&func.block, args.as_ref(), &func.return_type)
            .unwrap()
    }
}
