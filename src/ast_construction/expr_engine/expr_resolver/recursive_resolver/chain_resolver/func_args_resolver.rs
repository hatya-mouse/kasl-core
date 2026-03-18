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
    Expr, FuncParam, Range, ScopeID, ScopeVar, VariableID, error::Ph,
    expr_engine::ExpressionResolver, scope_manager::VariableKind, symbol_table::NoTypeFuncCallArg,
};

impl ExpressionResolver<'_> {
    pub fn create_func_call_arg(
        &mut self,
        name: String,
        value: Expr,
        func_scope: &ScopeID,
        range: Range,
    ) -> VariableID {
        // Create a ScopeVar for the function call argument
        let scope_var = ScopeVar {
            name,
            value_type: value.value_type,
            def_val: None,
            range,
            var_kind: VariableKind::FuncCallArg,
        };
        // Register the ScopeVar and create the FuncCallArg
        self.prog_ctx
            .scope_registry
            .register_var(scope_var, func_scope)
    }

    pub fn resolve_func_call_args(
        &mut self,
        func_params: &[FuncParam],
        self_param: Option<Expr>,
        no_type_args: &[NoTypeFuncCallArg],
        func_scope: &ScopeID,
        func_call_range: Range,
    ) -> Option<Vec<VariableID>> {
        let mut slots: Vec<Option<VariableID>> = vec![None; func_params.len()];
        let mut next_unlabeled_index = 0;

        // If the function has a self parameter, add it to the first slot
        if let Some(self_param) = self_param {
            let self_arg_id = self.create_func_call_arg(
                "self".to_string(),
                self_param,
                func_scope,
                func_call_range,
            );
            slots[0] = Some(self_arg_id);
            next_unlabeled_index = 1;
        }

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
                        .duplicate_arg_is_given(no_type_arg.range, Ph::ExprEngine, label);
                    return None;
                }
                // If the label order is incorrect, throw an error
                if param_index <= next_unlabeled_index {
                    self.ec
                        .arg_order_incorrect(no_type_arg.range, Ph::ExprEngine, label);
                    return None;
                }

                let arg_id = self.create_func_call_arg(
                    func_params[param_index].name.clone(),
                    value,
                    func_scope,
                    no_type_arg.range,
                );
                slots[param_index] = Some(arg_id);
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

                let arg_id = self.create_func_call_arg(
                    func_params[next_unlabeled_index].name.clone(),
                    value,
                    func_scope,
                    no_type_arg.range,
                );
                slots[next_unlabeled_index] = Some(arg_id);
                next_unlabeled_index += 1;
            }
        }

        let mut resolved_args = Vec::new();
        for (param, slot) in func_params.iter().zip(slots) {
            match slot {
                Some(arg_id) => resolved_args.push(arg_id),
                None => match &param.def_val {
                    Some(def_val) => {
                        let arg_id = self.create_func_call_arg(
                            param.name.clone(),
                            def_val.clone(),
                            func_scope,
                            Range::zero(),
                        );
                        resolved_args.push(arg_id);
                    }
                    None => {
                        self.ec
                            .missing_arg(func_call_range, Ph::ExprEngine, &param.name);
                    }
                },
            }
        }

        // Check if the type of the arguments matches the type of the parameter
        for (resolved_arg, param) in resolved_args.iter().zip(func_params.iter()) {
            // The argument must exist in the scope registry
            let arg_var = self.prog_ctx.scope_registry.get_var(resolved_arg).unwrap();
            if arg_var.value_type != param.value_type {
                self.ec.arg_type_mismatch(
                    arg_var.range,
                    Ph::ExprEngine,
                    &param.name,
                    self.prog_ctx.type_registry.format_type(&param.value_type),
                    self.prog_ctx.type_registry.format_type(&arg_var.value_type),
                );
                return None;
            }
        }

        Some(resolved_args)
    }
}
