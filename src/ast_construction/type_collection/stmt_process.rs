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
    ParserDeclStmt, ParserDeclStmtKind, error::Ph, symbol_path, type_collection::TypeCollector,
    type_registry::StructDecl,
};

impl TypeCollector<'_> {
    pub fn process_stmt(&mut self, stmt: &ParserDeclStmt) {
        if let ParserDeclStmtKind::StructDecl { name, .. } = &stmt.kind {
            let path = symbol_path![name.clone()];

            // Check if the struct with the same name already exists
            if self.compilation_state.type_registry.has_struct(&path) {
                self.ec
                    .duplicate_struct_name(stmt.range, Ph::StructCollection, name);
            }

            // Create a new struct declaration
            let struct_decl = StructDecl::new(name.clone(), stmt.range);
            let id = self.name_space.generate_struct_id();
            self.compilation_state
                .type_registry
                .register_struct(struct_decl, path, id);
        }
    }
}
