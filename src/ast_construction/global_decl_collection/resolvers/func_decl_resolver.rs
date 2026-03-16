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
    Range,
    error::Ph,
    global_decl_collection::{GlobalDeclCollector, resolvers::FuncDeclInfo},
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_global_func_decl(&mut self, info: FuncDeclInfo<'_>, decl_range: Range) {
        // Check if is_static is not set
        if info.is_static {
            self.ec
                .global_func_cannot_be_static(decl_range, Ph::GlobalDeclCollection, info.name);
            return;
        }

        // Build the function node
        let Some(func) = self.build_func(
            false,
            info.is_static,
            info.name,
            info.params,
            info.return_type,
            decl_range,
        ) else {
            return;
        };

        // Check if a function with the same name already exists
        if self
            .prog_ctx
            .namespace_registry
            .is_name_used(&self.current_namespace, info.name)
        {
            self.ec
                .duplicate_func_name(decl_range, Ph::GlobalDeclCollection, info.name);
            return;
        }

        // Register the function
        let func_id = self
            .prog_ctx
            .func_ctx
            .register_global_func(self.current_namespace, func);

        // Register the function body to the function body map
        self.comp_data
            .func_body_map
            .register(func_id, info.body.to_vec());
    }
}
