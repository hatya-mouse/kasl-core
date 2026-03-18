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
    FuncParam, Function, ParserFuncParam, Range, ScopeID, SymbolPath,
    error::Ph,
    global_decl_collection::GlobalDeclCollector,
    symbol_table::{Block, FunctionType},
    type_registry::{PrimitiveType, ResolvedType},
};
use std::collections::HashSet;

impl GlobalDeclCollector<'_> {
    pub fn build_func(
        &mut self,
        func_type: FunctionType,
        name: &str,
        params: &[ParserFuncParam],
        return_type: &Option<SymbolPath>,
        decl_range: Range,
    ) -> Option<Function> {
        // Create a function block
        let global_scope_id = self
            .prog_ctx
            .scope_registry
            .get_global_scope_id(&self.current_namespace);
        let func_scope_id = self
            .prog_ctx
            .scope_registry
            .create_scope(Some(global_scope_id), decl_range);
        let block = Block::new(func_scope_id);

        // Resolve the function parameters
        let mut params = self.resolve_func_params(params, func_scope_id)?;

        // If the function is an instance function, add the type at the first parameter
        if let FunctionType::Instance(struct_id) = func_type {
            params.insert(
                0,
                FuncParam {
                    label: None,
                    name: "self".to_string(),
                    value_type: ResolvedType::Struct(struct_id),
                    def_val: None,
                    range: decl_range,
                },
            );
        }

        // Resolve the return type
        let return_type = match return_type {
            Some(path) => {
                let (namespace_id, type_name) = self
                    .prog_ctx
                    .namespace_registry
                    .resolve_namespace_from_path(path.clone());
                match self
                    .prog_ctx
                    .type_registry
                    .resolve_type(namespace_id, &type_name.to_string())
                {
                    Some(resolved) => resolved,
                    None => {
                        self.ec.type_not_found(
                            decl_range,
                            Ph::GlobalDeclCollection,
                            path.to_string(),
                        );
                        return None;
                    }
                }
            }
            None => ResolvedType::Primitive(PrimitiveType::Void),
        };

        Some(Function {
            name: name.to_string(),
            namespace_id: self.current_namespace,
            func_type,
            params,
            return_type,
            block,
            range: decl_range,
        })
    }

    pub fn resolve_func_params(
        &mut self,
        params: &[ParserFuncParam],
        func_scope_id: ScopeID,
    ) -> Option<Vec<FuncParam>> {
        let mut resolved_params = Vec::new();
        let mut used_param_names = HashSet::new();
        // Resolve each parameter
        for param in params {
            let resolved_param = self.resolve_func_param(param, func_scope_id)?;
            resolved_params.push(resolved_param);

            // Add the parameter name to the used names set
            if used_param_names.contains(&param.name) {
                self.ec
                    .duplicate_name(param.range, Ph::StatementCollection, &param.name);
            } else {
                used_param_names.insert(param.name.clone());
            }
        }
        Some(resolved_params)
    }

    pub fn resolve_func_param(
        &mut self,
        param: &ParserFuncParam,
        func_scope_id: ScopeID,
    ) -> Option<FuncParam> {
        // Check if the name is already in use in this scope
        if self
            .prog_ctx
            .scope_registry
            .is_name_used(&func_scope_id, &param.name)
        {
            self.ec
                .duplicate_name(param.range, Ph::StatementCollection, &param.name);
            return None;
        }

        if let Some(def_val) = &param.def_val {
            // Resolve the default value expression
            let resolved_def_val =
                self.resolve_def_val_global(&param.value_type, def_val, param.range)?;

            Some(FuncParam {
                label: param.label.clone(),
                name: param.name.clone(),
                value_type: resolved_def_val.value_type,
                def_val: Some(resolved_def_val),
                range: param.range,
            })
        } else if let Some(annotation_type) = &param.value_type {
            // If no default value is provided, use the annotation type
            let (namespace_id, type_name) = self
                .prog_ctx
                .namespace_registry
                .resolve_namespace_from_path(annotation_type.clone());
            let resolved_annotation_type = self
                .prog_ctx
                .type_registry
                .resolve_type(namespace_id, &type_name.to_string())?;

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
