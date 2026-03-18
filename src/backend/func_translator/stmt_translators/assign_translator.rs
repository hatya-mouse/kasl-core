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

use crate::{
    Expr, StructID, backend::func_translator::FuncTranslator, symbol_table::LValue,
    type_registry::ResolvedType,
};
use cranelift::prelude::{InstBuilder, MemFlags};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn translate_assign(&mut self, target: &LValue, value: &Expr) {
        // Translate the RHS value
        let rhs_value = self.translate_expr(value);

        // Get the Variable and the ScopeVar
        let var = self.scope_registry.get_var(&target.var_id);

        // Set the value to the variable depending on the value type
        if target.is_field {
            let addr = self.builder.use_var(var);
            match &target.value_type {
                ResolvedType::Primitive(_) => {
                    self.builder
                        .ins()
                        .store(MemFlags::new(), rhs_value, addr, target.offset);
                }
                ResolvedType::Struct(struct_id) => {
                    self.copy_struct(struct_id, rhs_value, addr, target.offset);
                }
            }
        } else {
            self.builder.def_var(var, rhs_value);
        }
    }

    pub fn copy_struct(
        &mut self,
        struct_id: &StructID,
        src: ir::Value,
        dst: ir::Value,
        base_offset: i32,
    ) {
        let struct_decl = self.prog_ctx.type_registry.get_struct(struct_id).unwrap();
        for (field, offset) in struct_decl
            .fields
            .iter()
            .zip(struct_decl.field_offsets.iter())
        {
            match &field.value_type {
                ResolvedType::Primitive(_) => {
                    let ir_type = self.type_converter.convert(&field.value_type);
                    let val = self
                        .builder
                        .ins()
                        .load(ir_type, MemFlags::new(), src, *offset);
                    self.builder
                        .ins()
                        .store(MemFlags::new(), val, dst, base_offset + *offset);
                }
                ResolvedType::Struct(inner_id) => {
                    self.copy_struct(inner_id, src, dst, base_offset + offset);
                }
            }
        }
    }
}
