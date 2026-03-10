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
    FuncParam, Function, ParserFuncParam, Range, SymbolPath, error::Ph, expr_engine::resolve_expr,
    global_decl_collection::GlobalDeclCollector,
};

impl GlobalDeclCollector<'_> {
    pub fn build_func(
        &mut self,
        is_member: bool,
        is_static: bool,
        name: &str,
        params: &[ParserFuncParam],
        return_type: &Option<SymbolPath>,
        decl_range: Range,
    ) -> Option<Function> {
        // Resolve the function parameters
        let params = self.resolve_func_params(params)?;

        // Resolve the return type
        let return_type = match return_type {
            Some(path) => match self.type_registry.resolve_type_path(path) {
                Some(resolved) => Some(resolved),
                None => {
                    self.ec
                        .type_not_found(decl_range, Ph::GlobalDeclCollection, path.to_string());
                    return None;
                }
            },
            None => None,
        };

        Some(Function {
            name: name.to_string(),
            is_member,
            is_static,
            params,
            return_type,
            block: None,
            range: decl_range,
        })
    }

    pub fn resolve_func_params(&mut self, params: &[ParserFuncParam]) -> Option<Vec<FuncParam>> {
        let mut resolved_params = Vec::new();
        // Resolve each parameter
        for param in params {
            let resolved_param = self.resolve_func_param(param)?;
            resolved_params.push(resolved_param);
        }
        Some(resolved_params)
    }

    pub fn resolve_func_param(&mut self, param: &ParserFuncParam) -> Option<FuncParam> {
        let global_scope_id = self.scope_registry.get_global_scope_id();

        if let Some(def_val) = &param.def_val {
            // Resolve the default value expression
            let resolved_def_val = resolve_expr(
                self.ec,
                self.op_ctx,
                self.func_ctx,
                self.scope_registry,
                self.type_registry,
                global_scope_id,
                def_val,
            )?;

            // If a type annotation is provided, check if it matches the resolved default value type
            if let Some(annotation_type) = &param.value_type {
                let resolved_annotation_type =
                    self.type_registry.resolve_type_path(&annotation_type)?;
                if resolved_annotation_type != resolved_def_val.value_type {
                    self.ec.type_annotation_mismatch(
                        param.range,
                        Ph::GlobalDeclCollection,
                        self.type_registry.format_type(&resolved_annotation_type),
                        self.type_registry.format_type(&resolved_def_val.value_type),
                    );
                    return None;
                }
            }

            Some(FuncParam {
                label: param.label.clone(),
                name: param.name.clone(),
                value_type: resolved_def_val.value_type,
                def_val: Some(resolved_def_val),
                range: param.range,
            })
        } else if let Some(annotation_type) = &param.value_type {
            // If no default value is provided, use the annotation type
            let resolved_annotation_type =
                self.type_registry.resolve_type_path(&annotation_type)?;
            Some(FuncParam {
                label: param.label.clone(),
                name: param.name.clone(),
                value_type: resolved_annotation_type,
                def_val: None,
                range: param.range,
            })
        } else {
            self.ec
                .no_type_annotation_or_def_val(param.range, Ph::GlobalDeclCollection);
            None
        }
    }
}
