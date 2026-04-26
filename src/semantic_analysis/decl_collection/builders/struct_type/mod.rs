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

mod member_func_resolver;
mod struct_field_resolver;

use crate::{
    ast_nodes::{Range, StructID, symbol_table::FunctionType, type_registry::StructDecl},
    error::{EK, Ph},
    parser::{ParserDeclStmt, ParserDeclStmtKind},
    semantic_analysis::decl_collection::{GlobalDeclCollector, builders::FuncDeclInfo},
};

impl<'a> GlobalDeclCollector<'a> {
    pub fn resolve_struct_decl(
        &mut self,
        name: &str,
        body: &'a [ParserDeclStmt],
        decl_range: Range,
    ) {
        // Check if the struct with the same name already exists
        if self.is_name_used(name) {
            self.ec
                .duplicate_name(decl_range, Ph::StructCollection, name);
        }

        // Register the new struct
        let struct_id = self
            .prog_ctx
            .type_registry
            .register_struct(self.current_namespace, name.to_string());
        let mut struct_decl = StructDecl::new(name.to_string(), decl_range);

        // Create a new scope for the struct body and resolve the struct body
        let global_scope_id = self
            .prog_ctx
            .scope_registry
            .get_global_scope_id(&self.current_namespace);
        let struct_scope_id = self
            .prog_ctx
            .scope_registry
            .create_scope(Some(global_scope_id), decl_range);

        // Switch to the newly creates scope and resolve the struct body
        let old_scope_id = self.current_scope_id;
        self.switch_to_scope(struct_scope_id);
        self.resolve_struct_body(struct_id, &mut struct_decl, body);

        // Restore the old scope ID
        self.switch_to_scope(old_scope_id);

        // Calculate the struct layout
        if let Err(EK::StructCycle) = struct_decl.compute_layout(&self.prog_ctx.type_registry) {
            self.ec
                .struct_cycle(decl_range, Ph::GlobalDeclCollection, name);
        }

        // Register the struct decl in the type registry with a generated ID
        self.prog_ctx
            .type_registry
            .set_struct_decl(struct_id, struct_decl);

        // Mark the struct name as used in the namespace
        self.mark_name_used(name);
    }

    fn resolve_struct_body(
        &mut self,
        struct_id: StructID,
        struct_decl: &mut StructDecl,
        body: &'a [ParserDeclStmt],
    ) {
        for stmt in body {
            match &stmt.kind {
                ParserDeclStmtKind::StructField {
                    name,
                    value_type,
                    def_val,
                } => self.resolve_struct_field(
                    struct_id,
                    struct_decl,
                    name,
                    value_type,
                    def_val,
                    stmt.range,
                ),

                ParserDeclStmtKind::FuncDecl {
                    is_static,
                    name,
                    params,
                    return_type,
                    body,
                } => {
                    let func_type = if *is_static {
                        FunctionType::Static
                    } else {
                        FunctionType::Instance(struct_id)
                    };

                    let info = FuncDeclInfo {
                        func_type,
                        name,
                        params,
                        return_type,
                        body,
                    };
                    self.resolve_member_func_decl(struct_id, stmt.range, info);
                }

                _ => {
                    self.ec.invalid_struct_stmt(
                        stmt.range,
                        Ph::GlobalDeclCollection,
                        stmt.kind.to_string(),
                    );
                }
            }
        }
    }
}
