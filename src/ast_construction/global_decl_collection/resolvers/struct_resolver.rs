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
    ExprToken, ParserDeclStmt, ParserDeclStmtKind, Range, StructID, SymbolPath,
    error::Ph,
    global_decl_collection::{GlobalDeclCollector, resolvers::FuncDeclInfo},
    namespace_registry::is_reserved_type_name,
    symbol_path,
    type_registry::{ResolvedType, StructDecl, StructField},
};

impl<'a> GlobalDeclCollector<'a> {
    pub fn resolve_struct_decl(
        &mut self,
        name: &str,
        body: &'a [ParserDeclStmt],
        decl_range: Range,
    ) {
        let struct_path = symbol_path![name.to_string()];
        // Check if the struct with the same name already exists
        if self.namespace.type_registry.has_struct(&struct_path) {
            self.ec
                .duplicate_struct_name(decl_range, Ph::StructCollection, name);
        } else if is_reserved_type_name(name) {
            self.ec
                .reserved_struct_name(decl_range, Ph::StructCollection, name);
        }

        let struct_id = self.namespace.type_registry.generate_struct_id();
        let mut struct_decl = StructDecl::new(name.to_string(), decl_range);
        self.resolve_struct_body(struct_id, &mut struct_decl, body);

        // Calculate the struct layout
        struct_decl.compute_layout(&self.namespace.type_registry);

        // Register the struct in the type registry with a generated ID
        self.namespace
            .type_registry
            .register_struct(struct_decl, struct_path, struct_id);
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
                    let info = FuncDeclInfo {
                        is_static: *is_static,
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

    fn resolve_struct_field(
        &mut self,
        struct_id: StructID,
        struct_decl: &mut StructDecl,
        name: &str,
        value_type: &Option<SymbolPath>,
        def_val: &[ExprToken],
        decl_range: Range,
    ) {
        // Resolve the default value expression
        let Some(def_val) = self.resolve_def_val_global(value_type, def_val, decl_range) else {
            return;
        };

        // Add the field to the struct graph if the value has a struct type
        // The graph will be used in the scope graph analyzing phase to check if there aren't any struct cycles
        if let ResolvedType::Struct(field_struct_id) = def_val.value_type {
            self.comp_data
                .struct_graph
                .add_edge(struct_id, field_struct_id);
        }

        // Create a struct field
        let struct_field = StructField {
            name: name.to_string(),
            value_type: def_val.value_type,
            def_val,
            range: decl_range,
        };

        // Register the struct field in the type registry
        struct_decl.register_field(struct_field);
    }

    fn resolve_member_func_decl(
        &mut self,
        struct_id: StructID,
        decl_range: Range,
        info: FuncDeclInfo<'_>,
    ) {
        // Build the function node
        let Some(func) = self.build_func(
            true,
            info.is_static,
            info.name,
            info.params,
            info.return_type,
            decl_range,
        ) else {
            return;
        };

        // Register the function
        let func_id = self
            .namespace
            .func_ctx
            .register_member_func(func, struct_id);

        // Register the function body to the func body map
        self.comp_data
            .func_body_map
            .register(func_id, info.body.to_vec());
    }
}
