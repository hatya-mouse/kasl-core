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
    ast::{Expr, FuncCallArg, FuncParam, Range, symbol_table::NoTypeFuncCallArg},
    ast_construction::expr_engine::ExpressionResolver,
    error::Ph,
};

impl ExpressionResolver<'_> {
    pub fn resolve_func_call_args(
        &mut self,
        func_params: &[FuncParam],
        self_param: Option<Expr>,
        no_type_args: &[NoTypeFuncCallArg],
        func_call_range: Range,
    ) -> Option<Vec<FuncCallArg>> {
        let mut slots: Vec<Option<FuncCallArg>> = vec![None; func_params.len()];
        let mut next_unlabeled_index = 0;

        // If the function has a self parameter, add it to the first slot
        if let Some(self_param) = self_param {
            slots[0] = Some(FuncCallArg {
                var_id: func_params[0].var_id,
                value: self_param,
                range: func_call_range,
            });
            next_unlabeled_index = 1;
        }

        for no_type_arg in no_type_args {
            // Resolve the type of the argument expression
            let Some(value) = self.resolve_recursively(no_type_arg.value.clone()) else {
                continue;
            };

            if let Some(label) = &no_type_arg.label {
                // Resolve the type and check order
                let Some(param_index) = func_params
                    .iter()
                    .position(|p| p.label.as_ref().is_some_and(|l| l == label))
                else {
                    self.ec
                        .arg_label_not_found(no_type_arg.range, Ph::ExprEngine, label);
                    return None;
                };

                // If the slot is already occupied, throw an duplicate parameter error
                if slots[param_index].is_some() {
                    self.ec
                        .duplicate_arg_is_given(no_type_arg.range, Ph::ExprEngine, label);
                    return None;
                }
                // If the label order is incorrect, throw an error
                if param_index < next_unlabeled_index {
                    self.ec
                        .arg_order_incorrect(no_type_arg.range, Ph::ExprEngine, label);
                    return None;
                }

                slots[param_index] = Some(FuncCallArg {
                    var_id: func_params[param_index].var_id,
                    value,
                    range: no_type_arg.range,
                });
                next_unlabeled_index = param_index + 1;
            } else {
                // Check if the index is within bounds
                if next_unlabeled_index >= slots.len() {
                    self.ec
                        .extra_arg(no_type_arg.range, Ph::ExprEngine, slots.len());
                    return None;
                }
                // Check if the target argument doesn't require a label
                if func_params[next_unlabeled_index].label.is_some() {
                    self.ec.missing_arg_label(
                        no_type_arg.range,
                        Ph::ExprEngine,
                        &func_params[next_unlabeled_index].name,
                        func_params[next_unlabeled_index].label.as_ref().unwrap(),
                    );
                }

                slots[next_unlabeled_index] = Some(FuncCallArg {
                    var_id: func_params[next_unlabeled_index].var_id,
                    value,
                    range: no_type_arg.range,
                });
                next_unlabeled_index += 1;
            }
        }

        let mut resolved_args = Vec::new();
        for (slot, param) in slots.iter().zip(func_params.iter()) {
            match slot {
                Some(arg) => resolved_args.push(arg.clone()),
                None => match &param.def_val {
                    Some(def_val) => resolved_args.push(FuncCallArg {
                        var_id: param.var_id,
                        value: def_val.clone(),
                        range: Range::zero(),
                    }),
                    None => {
                        self.ec
                            .missing_arg(func_call_range, Ph::ExprEngine, &param.name);
                    }
                },
            }
        }

        // Check if the type of the arguments matches the type of the parameter
        for (resolved_arg, param) in resolved_args.iter().zip(func_params.iter()) {
            if resolved_arg.value.value_type != param.value_type {
                self.ec.arg_type_mismatch(
                    resolved_arg.range,
                    Ph::ExprEngine,
                    &param.name,
                    self.prog_ctx.type_registry.format_type(&param.value_type),
                    self.prog_ctx
                        .type_registry
                        .format_type(&resolved_arg.value.value_type),
                );
                return None;
            }
        }

        Some(resolved_args)
    }
}
