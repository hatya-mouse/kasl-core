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
    Function, Initializer, LiteralBind, ParserFuncParam, ParserSymbolPath, Range, SymbolPath,
    error::Phase, resolution::TypeResolveCtx,
};

impl<'a> TypeResolveCtx<'a> {
    pub fn resolve_func(
        &mut self,
        name: &str,
        symbol_path: &SymbolPath,
        params: &[ParserFuncParam],
        required_by: Option<&ParserSymbolPath>,
        return_type: Option<&ParserSymbolPath>,
        decl_range: Range,
    ) {
        // If the function has required-by type, resolve the type
        let resolved_required_by = match required_by {
            Some(required_by) => match self.program.resolve_type_def_parser_path(required_by) {
                Some(resolved_path) => Some(resolved_path),
                None => {
                    self.ec.type_not_found(
                        decl_range,
                        Phase::TypeResolution,
                        &required_by.to_string(),
                    );
                    None
                }
            },
            None => None,
        };

        // If the function has a return type, resolve the type
        let resolved_return_type = match return_type {
            Some(return_type) => match self.program.resolve_type_def_parser_path(return_type) {
                Some(resolved_path) => Some(resolved_path),
                None => {
                    self.ec.type_not_found(
                        decl_range,
                        Phase::TypeResolution,
                        &return_type.to_string(),
                    );
                    None
                }
            },
            None => None,
        };

        // Resolve the variables
        let mut resolved_params = Vec::new();
        for param in params {
            match self.resolve_param(param) {
                Some(param) => resolved_params.push(param),
                None => return,
            }
        }

        // Construct a function and push it to the program
        let func = Function {
            name: name.to_string(),
            params: resolved_params,
            return_type: resolved_return_type,
            required_by: resolved_required_by,
            body: Vec::new(),
        };

        // Obtain the path to the parent scope of the function
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

        // Register the function to the Program
        self.program
            .register_func_by_path(self.ec, func, &parent_path, decl_range);
    }

    pub fn resolve_init(
        &mut self,
        symbol_path: &SymbolPath,
        literal_bind: Option<&LiteralBind>,
        params: &[ParserFuncParam],
        required_by: Option<&ParserSymbolPath>,
        decl_range: Range,
    ) {
        // If the function has required-by type, resolve the type
        let resolved_required_by = match required_by {
            Some(required_by) => match self.program.resolve_type_def_parser_path(required_by) {
                Some(resolved_path) => Some(resolved_path),
                None => {
                    self.ec.type_not_found(
                        decl_range,
                        Phase::TypeResolution,
                        &required_by.to_string(),
                    );
                    None
                }
            },
            None => None,
        };

        // Resolve the variables
        let mut resolved_params = Vec::new();
        for param in params {
            match self.resolve_param(param) {
                Some(param) => resolved_params.push(param),
                None => return,
            }
        }

        // Construct a function and push it to the program
        let init = Initializer {
            literal_bind: literal_bind.cloned(),
            params: resolved_params,
            required_by: resolved_required_by,
            body: Vec::new(),
        };

        // Obtain the path to the parent scope of the function
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

        // Register the function to the Program
        self.program
            .register_init_by_path(self.ec, init, &parent_path, decl_range);

        // Register the intializer as a literal bind if it is specified
        if let Some(literal_bind) = literal_bind {
            match literal_bind {
                LiteralBind::IntLiteral => {
                    self.program
                        .set_int_literal(self.ec, parent_path, decl_range);
                }
                LiteralBind::FloatLiteral => {
                    self.program
                        .set_float_literal(self.ec, parent_path, decl_range)
                }
                LiteralBind::BoolLiteral => {
                    self.program
                        .set_bool_literal(self.ec, parent_path, decl_range)
                }
            }
        }
    }
}
