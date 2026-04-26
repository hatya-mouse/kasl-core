//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::{
    ast_nodes::{
        Expr,
        symbol_table::{LValue, LValueKind},
        type_registry::ResolvedType,
    },
    lowerer::func_translator::FuncTranslator,
};
use kasl_ir::{Const, InstBuilder, IntBinOp, Offset, Value};

impl FuncTranslator<'_> {
    pub fn translate_assign(&mut self, target: &LValue, value: &Expr) {
        // Translate the RHS value
        let rhs_value = self.translate_expr(value).unwrap();

        // Set the value to the variable depending on the kind of l-value
        if let LValueKind::Identifier(var_id) = &target.kind {
            let var = self.scope_registry.get_var(var_id);
            self.builder.assign(var, rhs_value);
        } else {
            let val_ptr = self.resolve_l_value_ptr(target);
            match &target.value_type {
                ResolvedType::Primitive(_) => {
                    self.builder.store(rhs_value, val_ptr, Offset::zero());
                }
                ResolvedType::Struct(struct_id) => {
                    self.copy_struct(
                        struct_id,
                        rhs_value,
                        Offset::zero(),
                        val_ptr,
                        Offset::zero(),
                    );
                }
                ResolvedType::Array(array_id) => {
                    self.copy_array(array_id, rhs_value, Offset::zero(), val_ptr, Offset::zero());
                }
            }
        }
    }

    fn resolve_l_value_ptr(&mut self, l_value: &LValue) -> Value {
        match &l_value.kind {
            LValueKind::Identifier(var_id) => {
                // Get the address to the value stored in the stack slot
                let var = self.scope_registry.get_var(var_id);
                self.builder.load_var(var)
            }
            LValueKind::Subscript { lhs, index } => {
                // Resolve the lhs to get the pointer to the array itself
                let base_ptr = self.resolve_l_value_ptr(lhs);
                // Calculate the pointer to the corresponding value
                self.calculate_array_offset(&lhs.value_type, base_ptr, index)
            }
            LValueKind::StructField { lhs, offset } => {
                // Resolve the lhs to get the pointer to the struct
                let base_ptr = self.resolve_l_value_ptr(lhs);
                // Add the offset to the base pointer
                let offset_const = self.builder.const_val(Const::Ptr(*offset as i64));
                self.builder.ibop(IntBinOp::Add, base_ptr, offset_const)
            }
        }
    }
}
