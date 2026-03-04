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
    ExprToken, InputAttribute, InputVar, OutputVar, ParserInputAttribute, ParserSymbolPath, Range,
    ScopeVar, StateVar, SymbolPath,
    error::Phase,
    resolution::{TypeResolveCtx, expr_inference::ExprTreeBuilder},
};

impl<'a> TypeResolveCtx<'a> {
    pub fn resolve_input(
        &mut self,
        name: &str,
        value_type: Option<&ParserSymbolPath>,
        def_val: &[ExprToken],
        parser_attrs: &[ParserInputAttribute],
        decl_range: Range,
    ) {
        if let Some((value_type, def_val)) = self.resolve_var_type(decl_range, value_type, def_val)
        {
            // Parser the attributes one by one
            let mut attrs = Vec::new();
            for parser_attr in parser_attrs {
                let args = parser_attr
                    .args
                    .iter()
                    .flat_map(|expr| {
                        self.program.build_expr_tree_from_raw_tokens(
                            self.ec,
                            expr,
                            self.symbol_table,
                        )
                    })
                    .collect::<Vec<_>>();
                let attr = InputAttribute {
                    name: parser_attr.name.clone(),
                    args,
                };
                attrs.push(attr);
            }

            // Create an input variable and push it to the program
            let input = InputVar {
                name: name.to_string(),
                value_type,
                def_val,
                attrs,
            };
            self.program.register_input(input);
        }
    }

    pub fn resolve_output(
        &mut self,
        name: &str,
        value_type: Option<&ParserSymbolPath>,
        def_val: &[ExprToken],
        decl_range: Range,
    ) {
        if let Some((value_type, def_val)) = self.resolve_var_type(decl_range, value_type, def_val)
        {
            // Create an output variable and push it to the program
            let output = OutputVar {
                name: name.to_string(),
                value_type,
                def_val,
            };
            self.program.register_output(output);
        }
    }

    pub fn resolve_state(
        &mut self,
        name: &str,
        value_type: Option<&ParserSymbolPath>,
        def_val: &[ExprToken],
        decl_range: Range,
    ) {
        if let Some((value_type, def_val)) = self.resolve_var_type(decl_range, value_type, def_val)
        {
            // Create a state variable and push it to the program
            let state = StateVar {
                name: name.to_string(),
                value_type,
                def_val,
            };
            self.program.register_state(state);
        }
    }

    pub fn resolve_var(
        &mut self,
        name: &str,
        symbol_path: &SymbolPath,
        value_type: Option<&ParserSymbolPath>,
        def_val: &[ExprToken],
        decl_range: Range,
    ) {
        if let Some((value_type, def_val)) = self.resolve_var_type(decl_range, value_type, def_val)
        {
            // Create a scope variable and push it to the program
            let var = ScopeVar {
                name: name.to_string(),
                value_type,
                def_val,
            };

            // Obtain the path to the parent scope of the variable
            let parent_path = match symbol_path.parent() {
                Some(path) => path,
                None => {
                    self.ec.comp_bug(
                        decl_range,
                        Phase::TypeResolution,
                        "The symbol path should include one component at least.",
                    );
                    return;
                }
            };

            // Register the variable in the parent scope
            self.program
                .register_var_by_path(self.ec, var, &parent_path, decl_range);
        }
    }
}
