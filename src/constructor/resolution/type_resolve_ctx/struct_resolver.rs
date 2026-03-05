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

use crate::{StructDecl, data::ParserStmtID, resolution::type_resolve_ctx::TypeResolveCtx};

impl<'a> TypeResolveCtx<'a> {
    pub fn register_struct(&mut self, symbol_id: &ParserStmtID, name: &str) {
        if let Some(path) = self.symbol_table.get_path_by_id(symbol_id) {
            let struct_decl = StructDecl {
                name: name.to_string(),
            };
            self.program.register_struct_decl(struct_decl, path.clone());
        }
    }
}
