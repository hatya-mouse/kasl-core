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

use crate::{Expr, backend::func_translator::FuncTranslator, builtin::BuiltinFuncID};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub(super) fn translate_builtin_func_call(
        &mut self,
        func_id: &BuiltinFuncID,
        args: &[Expr],
    ) -> ir::Value {
        // Translate the expressions
        let mut translated_args = Vec::new();
        for arg in args {
            let translated_val = self.translate_expr(arg).unwrap();
            translated_args.push(translated_val);
        }

        // Get a reference to the function
        let func = &self.builtin_registry.get_func_by_id(func_id).unwrap();

        // Translate the function
        (func.translator)(&mut self.builder, &translated_args)
    }
}
