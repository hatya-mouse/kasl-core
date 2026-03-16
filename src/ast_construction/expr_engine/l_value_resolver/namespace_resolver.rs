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

use crate::{ExprToken, ExprTokenKind, ScopeID, expr_engine::LValueResolver};
use std::{iter::Peekable, slice::Iter};

impl LValueResolver<'_> {
    pub fn resolve_namespace_scope(&mut self, tokens: &mut Peekable<Iter<ExprToken>>) -> ScopeID {
        let mut current_scope = self.current_scope;
        let mut current_namespace = self.current_namespace;
        // Loop over the elements in the chain and get the namespace ID from the first tokens
        while let Some(token) = tokens.peek() {
            match &token.kind {
                ExprTokenKind::Identifier(name) => {
                    let namespace_ref = self
                        .prog_ctx
                        .namespace_registry
                        .get_namespace_by_id(&current_namespace)
                        .unwrap();
                    if let Some(namespace_id) = namespace_ref.get_id_by_name(name) {
                        // Consume the identifier
                        tokens.next();
                        current_namespace = namespace_id;
                        // Get the global scope of the namespace
                        let global_scope = self
                            .prog_ctx
                            .scope_registry
                            .get_global_scope_id(&namespace_id);
                        current_scope = global_scope;
                    } else {
                        return current_scope;
                    }
                }
                _ => return current_scope,
            }
        }
        current_scope
    }
}
