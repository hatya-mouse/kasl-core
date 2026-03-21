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
    Expr, StructID,
    backend::func_translator::FuncTranslator,
    namespace_registry::ArrayID,
    symbol_table::{LValue, LValueKind},
    type_registry::ResolvedType,
};
use cranelift::prelude::{InstBuilder, MemFlags};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn translate_assign(&mut self, target: &LValue, value: &Expr) {
        // Translate the RHS value
        let rhs_value = self.translate_expr(value);

        // Set the value to the variable depending on the l-value kind
        if let LValueKind::Identifier(var_id) = &target.kind {
            let var = self.scope_registry.get_var(var_id);
            self.builder.def_var(var, rhs_value);
        } else {
            let val_ptr = self.resolve_l_value_ptr(target);
            match &target.value_type {
                ResolvedType::Primitive(_) => {
                    self.builder
                        .ins()
                        .store(MemFlags::new(), rhs_value, val_ptr, 0);
                }
                ResolvedType::Struct(struct_id) => {
                    self.copy_struct(struct_id, rhs_value, val_ptr, 0);
                }
                ResolvedType::Array(array_id) => {
                    self.copy_array(array_id, rhs_value, val_ptr, 0);
                }
            }
        }
    }

    fn resolve_l_value_ptr(&mut self, l_value: &LValue) -> ir::Value {
        match &l_value.kind {
            LValueKind::Identifier(var_id) => {
                // Get the address to the value stored in the stack slot
                let var = self.scope_registry.get_var(var_id);
                self.builder.use_var(var)
            }
            LValueKind::Subscript { lhs, index } => {
                // Resolve the lhs to get the pointer to the array itself
                let base_ptr = self.resolve_l_value_ptr(lhs);
                // Calculate the pointer to the corresponding value
                self.calculate_array_offset(&l_value.value_type, base_ptr, index)
            }
            LValueKind::StructField { lhs, offset } => {
                // Resolve the lhs to get the pointer to the struct
                let base_ptr = self.resolve_l_value_ptr(lhs);
                // Add the offset to the base pointer
                let offset_const = self
                    .builder
                    .ins()
                    .iconst(self.type_converter.pointer_type(), *offset as i64);
                self.builder.ins().iadd(base_ptr, offset_const)
            }
        }
    }

    fn copy_struct(
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
            self.copy_value(&field.value_type, src, dst, base_offset, *offset);
        }
    }

    fn copy_array(&mut self, array_id: &ArrayID, src: ir::Value, dst: ir::Value, base_offset: i32) {
        let array_decl = self
            .prog_ctx
            .type_registry
            .get_array_decl(array_id)
            .unwrap();
        // Get the size of the item type from the type registry
        let item_type = array_decl.item_type();
        let item_size = self
            .prog_ctx
            .type_registry
            .get_type_actual_size(item_type)
            .unwrap() as i32;

        let mut offset: i32 = 0;
        for _ in 0..*array_decl.count() {
            self.copy_value(item_type, src, dst, base_offset, offset);
            // Increment the offset every iteration
            offset += item_size;
        }
    }

    fn copy_value(
        &mut self,
        value_type: &ResolvedType,
        src: ir::Value,
        dst: ir::Value,
        base_offset: i32,
        offset: i32,
    ) {
        match value_type {
            ResolvedType::Primitive(_) => {
                let ir_type = self.type_converter.convert(value_type);
                let val = self
                    .builder
                    .ins()
                    .load(ir_type, MemFlags::new(), src, offset);
                self.builder
                    .ins()
                    .store(MemFlags::new(), val, dst, base_offset + offset);
            }
            ResolvedType::Array(inner_id) => {
                self.copy_array(inner_id, src, dst, base_offset + offset);
            }
            ResolvedType::Struct(inner_id) => {
                self.copy_struct(inner_id, src, dst, base_offset + offset);
            }
        }
    }
}
