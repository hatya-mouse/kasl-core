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
    FuncParam, Function, ParserFuncParam, Range, ScopeID, ScopeVar, SymbolPath,
    error::Ph,
    global_decl_collection::GlobalDeclCollector,
    scope_manager::VariableKind,
    symbol_table::Block,
    type_registry::{PrimitiveType, ResolvedType},
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
        // Create a function block
        let global_scope_id = self.comp_state.scope_registry.get_global_scope_id();
        let func_scope_id = self
            .comp_state
            .scope_registry
            .create_scope(Some(global_scope_id), decl_range);
        let block = Block::new(func_scope_id);

        // Resolve the function parameters
        let params = self.resolve_func_params(params, func_scope_id)?;

        // Resolve the return type
        let return_type = match return_type {
            Some(path) => match self.comp_state.type_registry.resolve_type_path(path) {
                Some(resolved) => resolved,
                None => {
                    self.ec
                        .type_not_found(decl_range, Ph::GlobalDeclCollection, path.to_string());
                    return None;
                }
            },
            None => ResolvedType::Primitive(PrimitiveType::Void),
        };

        Some(Function {
            name: name.to_string(),
            is_member,
            is_static,
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
        // Resolve each parameter
        for param in params {
            let resolved_param = self.resolve_func_param(param, func_scope_id)?;
            resolved_params.push(resolved_param);
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
            .comp_state
            .scope_registry
            .has_var(func_scope_id, &param.name)
        {
            self.ec
                .duplicate_var_name(param.range, Ph::StatementCollection, &param.name);
            return None;
        }

        if let Some(def_val) = &param.def_val {
            // Resolve the default value expression
            let resolved_def_val =
                self.resolve_def_val_global(&param.value_type, def_val, param.range)?;

            // Register the variable in the function scope
            let var = ScopeVar {
                name: param.name.clone(),
                value_type: resolved_def_val.value_type,
                def_val: None,
                range: param.range,
                var_kind: VariableKind::FuncParam,
            };
            let variable_id = self.name_space.generate_variable_id();
            self.comp_state.scope_registry.register_var(
                var,
                param.name.clone(),
                variable_id,
                func_scope_id,
            );

            Some(FuncParam {
                label: param.label.clone(),
                name: param.name.clone(),
                var_id: variable_id,
                value_type: resolved_def_val.value_type,
                def_val: Some(resolved_def_val),
                range: param.range,
            })
        } else if let Some(annotation_type) = &param.value_type {
            // If no default value is provided, use the annotation type
            let resolved_annotation_type = self
                .comp_state
                .type_registry
                .resolve_type_path(annotation_type)?;

            // Register the variable in the function scope
            let var = ScopeVar {
                name: param.name.clone(),
                value_type: resolved_annotation_type,
                def_val: None,
                range: param.range,
                var_kind: VariableKind::FuncParam,
            };
            let variable_id = self.name_space.generate_variable_id();
            self.comp_state.scope_registry.register_var(
                var,
                param.name.clone(),
                variable_id,
                func_scope_id,
            );

            Some(FuncParam {
                label: param.label.clone(),
                name: param.name.clone(),
                var_id: variable_id,
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
