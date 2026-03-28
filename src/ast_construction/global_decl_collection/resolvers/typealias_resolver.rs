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
    ast::Range,
    ast_construction::{common_utils::resolve_type, global_decl_collection::GlobalDeclCollector},
    error::Ph,
    parser::parser_ast::ParserTypeName,
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_typealias(&mut self, alias: &str, target: &ParserTypeName, decl_range: Range) {
        // Resolve the target type
        let resolved_target = match resolve_type(self.current_namespace, self.prog_ctx, target) {
            Some(ty) => ty,
            None => {
                self.ec
                    .type_not_found(decl_range, Ph::GlobalDeclCollection, target.to_string());
                return;
            }
        };

        // Add the typeealias to the type registry
        self.prog_ctx.type_registry.register_typealias(
            self.current_namespace,
            alias.to_string(),
            resolved_target,
        );
        // Mark the typealias as used in the namespace
        self.mark_name_used(alias);
    }
}
