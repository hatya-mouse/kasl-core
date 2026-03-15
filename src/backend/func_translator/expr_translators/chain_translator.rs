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

use cranelift::prelude::{InstBuilder, MemFlags};
use cranelift_codegen::ir;

use crate::{
    Expr, backend::func_translator::FuncTranslator, symbol_table::MemberAccess,
    type_registry::ResolvedType,
};

impl FuncTranslator<'_> {
    pub fn translate_chain(
        &mut self,
        lhs: &Expr<ResolvedType>,
        access: &MemberAccess,
        value_type: &ResolvedType,
    ) -> Option<ir::Value> {
        // Translate the expression
        let translated_lhs = self.translate_expr(lhs).unwrap();
        // Translate the type
        let translated_type = self.type_converter.convert(value_type);

        // Get the value depending on the access
        match access {
            MemberAccess::Access { offset, .. } => Some(self.builder.ins().load(
                translated_type,
                MemFlags::new(),
                translated_lhs,
                offset.unwrap(),
            )),
            MemberAccess::FuncCall { id, args, .. } => {
                let func = self.namespace.func_ctx.get_func(&id.unwrap()).unwrap();
                self.call_func(&func.block, args.as_ref().unwrap(), &func.return_type)
            }
        }
    }
}
