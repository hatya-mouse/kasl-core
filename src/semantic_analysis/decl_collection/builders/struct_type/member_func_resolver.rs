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
    ast_nodes::{Range, StructID},
    semantic_analysis::decl_collection::{GlobalDeclCollector, builders::FuncDeclInfo},
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_member_func_decl(
        &mut self,
        struct_id: StructID,
        decl_range: Range,
        info: FuncDeclInfo<'_>,
    ) {
        // Build the function node
        let Some(func) = self.build_func(
            info.func_type,
            info.name,
            info.params,
            info.return_type,
            decl_range,
        ) else {
            return;
        };

        // Register the function
        let func_id = self.prog_ctx.func_ctx.register_member_func(func, struct_id);

        // Mark the function name as used in the namespace
        self.mark_name_used(info.name);

        // Register the function body to the func body map
        self.comp_data
            .func_body_map
            .register(func_id, info.body.to_vec());
    }
}
