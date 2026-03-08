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
    ParserFuncParam, ParserScopeStmt, Range, SymbolPath, error::Ph,
    global_decl_collection::GlobalDeclCollector,
};

impl<'a> GlobalDeclCollector<'a> {
    pub fn resolve_global_func_decl(
        &mut self,
        is_static: bool,
        name: &str,
        params: &[ParserFuncParam],
        return_type: &Option<SymbolPath>,
        body: &'a Vec<ParserScopeStmt>,
        decl_range: Range,
    ) {
        // Check if is_static is not set
        if is_static {
            self.ec
                .global_func_cannot_be_static(decl_range, Ph::GlobalDeclCollection, name);
            return;
        }

        // Build the function node
        let Some(func) = self.build_func(false, is_static, name, params, return_type, decl_range)
        else {
            return;
        };

        // Register the function
        let func_id = self.name_space.generate_function_id();
        self.func_ctx.register_global_func(func, func_id);

        // Register the function body to the function body map
        self.func_body_map.register(func_id, body);
    }
}
