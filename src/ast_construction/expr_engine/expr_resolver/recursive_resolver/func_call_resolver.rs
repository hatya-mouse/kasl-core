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
    Expr, ExprKind, FuncCallArg, FuncParam, Range, error::Ph, expr_engine::ExpressionResolver,
    symbol_path, symbol_table::NoTypeFuncCallArg, type_registry::ResolvedType,
};

impl ExpressionResolver<'_> {
    pub fn resolve_func_call(
        &mut self,
        name: String,
        no_type_args: Vec<NoTypeFuncCallArg>,
        range: Range,
    ) -> Option<Expr<ResolvedType>> {
        if let Some(func_id) = self.comp_state.func_ctx.get_global_func_by_name(&name) {
            // Get a reference to the function
            let func = self.comp_state.func_ctx.get_func(&func_id)?;
            let args = self.resolve_func_call_args(&func.params, &no_type_args, range)?;

            // Add a function call edge to the scope graph
            // This is used to detect recursion
            self.scope_graph
                .add_edge(self.current_scope, func.block.scope_id);

            Some(Expr::new(
                ExprKind::FuncCall {
                    name,
                    id: Some(func_id),
                    no_type_args,
                    args: Some(args),
                },
                func.return_type,
                range,
            ))
        } else if let Some(struct_id) = self
            .comp_state
            .type_registry
            .get_struct_id_by_path(&symbol_path![name.clone()])
        {
            // If the function does not exist, check if the type with the same name exists
            // Ensure that the function doesn't have any arguments
            if !no_type_args.is_empty() {
                self.ec.arg_for_struct_init(range, Ph::ExprEngine);
            }

            Some(Expr::new(
                ExprKind::StructInit {
                    name,
                    id: struct_id,
                },
                ResolvedType::Struct(struct_id),
                range,
            ))
        } else {
            self.ec.func_not_found(range, Ph::ExprEngine, &name);
            None
        }
    }

    pub fn resolve_func_call_args(
        &mut self,
        func_params: &[FuncParam],
        no_type_args: &[NoTypeFuncCallArg],
        func_call_range: Range,
    ) -> Option<Vec<FuncCallArg>> {
        let mut slots: Vec<Option<FuncCallArg>> = vec![None; func_params.len()];
        let mut next_unlabeled_index = 0;

        for no_type_arg in no_type_args {
            // Resolve the type of the argument expression
            let Some(value) = self.resolve_recursively(no_type_arg.value.clone()) else {
                continue;
            };

            if let Some(label) = &no_type_arg.label {
                // Resolve the type and check order
                let param_index = func_params
                    .iter()
                    .position(|p| p.label.as_ref().is_some_and(|l| l == label))?;

                // If the slot is already occupied, throw an duplicate parameter error
                if slots[param_index].is_some() {
                    self.ec
                        .duplicate_arg(no_type_arg.range, Ph::ExprEngine, label);
                    return None;
                }
                // If the label order is incorrect, throw an error
                if param_index <= next_unlabeled_index {
                    self.ec
                        .arg_order_incorrect(no_type_arg.range, Ph::ExprEngine, label);
                    return None;
                }

                slots[param_index] = Some(FuncCallArg {
                    var_id: func_params[param_index].var_id,
                    value,
                });
                next_unlabeled_index = param_index + 1;
            } else {
                // Check if the index is within bounds
                if next_unlabeled_index >= slots.len() {
                    self.ec.extra_arg(no_type_arg.range, Ph::ExprEngine);
                    return None;
                }
                // Check if the target argument doesn't require a label
                if func_params[next_unlabeled_index].label.is_some() {
                    self.ec.missing_arg_label(no_type_arg.range, Ph::ExprEngine);
                    return None;
                }

                slots[next_unlabeled_index] = Some(FuncCallArg {
                    var_id: func_params[next_unlabeled_index].var_id,
                    value,
                });
                next_unlabeled_index += 1;
            }
        }

        let mut resolved_args = Vec::new();
        for (slot, param) in slots.iter().zip(func_params.iter()) {
            match slot {
                Some(arg) => resolved_args.push(arg.clone()),
                None => {
                    match param.def_val {
                        // If the parameter has a default value, use it
                        Some(ref def_val) => resolved_args.push(FuncCallArg {
                            var_id: param.var_id,
                            value: def_val.clone(),
                        }),
                        None => {
                            self.ec.missing_arg(func_call_range, Ph::ExprEngine);
                            return None;
                        }
                    }
                }
            }
        }

        Some(resolved_args)
    }
}
