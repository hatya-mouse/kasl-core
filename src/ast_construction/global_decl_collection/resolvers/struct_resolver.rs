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
    ExprToken, ParserDeclStmt, ParserDeclStmtKind, Range, SymbolPath,
    global_decl_collection::GlobalDeclCollector,
    symbol_path,
    type_registry::{StructDecl, StructField},
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_struct_decl(&mut self, name: &str, body: &[ParserDeclStmt], decl_range: Range) {
        let mut struct_decl = StructDecl::new(name.to_string(), decl_range);
        self.resolve_struct_body(&mut struct_decl, body);

        // Register the struct in the type registry with a generated ID
        let struct_path = symbol_path![name.to_string()];
        let struct_id = self.name_space.generate_struct_id();
        self.type_registry
            .register_struct(struct_decl, struct_path, struct_id);
    }

    fn resolve_struct_body(&mut self, struct_decl: &mut StructDecl, body: &[ParserDeclStmt]) {
        for stmt in body {
            match &stmt.kind {
                ParserDeclStmtKind::StructField {
                    name,
                    value_type,
                    def_val,
                } => {
                    self.resolve_struct_field(struct_decl, name, value_type, def_val, stmt.range);
                }

                ParserDeclStmtKind::FuncDecl {
                    is_static,
                    name,
                    params,
                    return_type,
                    body,
                } => {}

                _ => {
                    self.ec
                        .invalid_struct_stmt(stmt.range, stmt.kind.to_string());
                }
            }
        }
    }

    fn resolve_struct_field(
        &mut self,
        struct_decl: &mut StructDecl,
        name: &str,
        value_type: &Option<SymbolPath>,
        def_val: &Vec<ExprToken>,
        decl_range: Range,
    ) {
        // Resolve the default value expression
        let Some(def_val) = self.resolve_def_val_global(value_type, def_val, decl_range) else {
            return;
        };

        // Create a struct field
        let struct_field = StructField {
            name: name.to_string(),
            value_type: def_val.value_type.clone(),
            def_val,
            range: decl_range,
        };
        // Register the struct field in the type registry
        struct_decl.register_field(struct_field);
    }
}
