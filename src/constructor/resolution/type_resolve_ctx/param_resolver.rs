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
    FuncParam, ParserFuncParam,
    error::Phase,
    resolution::{TypeResolveCtx, expr_inference::ExprTreeBuilder},
};

impl<'a> TypeResolveCtx<'a> {
    pub fn resolve_param(&mut self, param: &ParserFuncParam) -> Option<FuncParam> {
        if let Some(value_type) = &param.value_type {
            // If the parameter has a type annotation, use it
            let resolved_type = match self
                .program
                .get_id_by_path(value_type)
                .and_then(|ids| ids.first().cloned())
            {
                Some(resolved_path) => resolved_path,
                None => {
                    self.ec.type_not_found(
                        param.range,
                        Phase::TypeResolution,
                        &value_type.to_string(),
                    );
                    return None;
                }
            };

            Some(FuncParam {
                label: param.label.clone(),
                name: param.name.clone(),
                value_type: resolved_type,
                def_val: None,
            })
        } else if let Some(def_val) = &param.def_val {
            // If the parameter does not have a type annotation, infer it from the expression
            let expr = self.program.build_expr_tree_from_raw_tokens(
                self.ec,
                def_val,
                self.symbol_table,
            )?;
            let resolved_type = expr.get_type(self.ec, self.program, param.range)?;

            // Construct the parameter
            Some(FuncParam {
                label: param.label.clone(),
                name: param.name.clone(),
                value_type: resolved_type,
                def_val: Some(expr),
            })
        } else {
            // If the parameter does not have a type annotation or default value, throw an error
            self.ec
                .param_without_type(param.range, Phase::TypeResolution, &param.name);
            None
        }
    }
}
