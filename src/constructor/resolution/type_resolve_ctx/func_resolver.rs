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
    Function, ParserFuncParam, Range, SymbolPath, data::ParserStmtID, error::Phase,
    resolution::TypeResolveCtx,
};

impl<'a> TypeResolveCtx<'a> {
    pub fn resolve_func(
        &mut self,
        is_static: bool,
        name: &str,
        symbol_id: &ParserStmtID,
        params: &[ParserFuncParam],
        return_type: Option<&SymbolPath>,
        decl_range: Range,
    ) {
        if let Some(path) = self.symbol_table.get_path_by_id(symbol_id) {
            // If the function has a return type, resolve the type
            let resolved_return_type = match return_type {
                Some(return_type) => match self
                    .program
                    .get_id_by_path(return_type)
                    .and_then(|ids| ids.first().cloned())
                {
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
                is_static,
                params: resolved_params,
                return_type: resolved_return_type,
                body: Vec::new(),
            };

            // Register the function to the Program
            self.program.register_func(func, path.clone());
        }
    }
}
