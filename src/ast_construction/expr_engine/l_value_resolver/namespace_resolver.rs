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
    ExprToken, ExprTokenKind, ScopeID, expr_engine::LValueResolver,
    namespace_registry::NameSpacePair,
};
use std::{iter::Peekable, slice::Iter};

impl LValueResolver<'_> {
    pub fn resolve_namespace_scope(
        &mut self,
        tokens: &mut Peekable<Iter<ExprToken>>,
    ) -> NameSpacePair<ScopeID> {
        let mut current_scope = self.current_scope;
        // Loop over the elements in the chain and get the namespace ID from the first tokens
        while let Some(token) = tokens.peek() {
            match &token.kind {
                ExprTokenKind::Identifier(name) => {
                    let current_namespace = self
                        .namespace_registry
                        .get_namespace_by_id(&current_scope.namespace_id)
                        .unwrap();
                    if let Some(namespace_id) = current_namespace.get_id_by_name(name)
                        && let Some(namespace) =
                            self.namespace_registry.get_namespace_by_id(&namespace_id)
                    {
                        // Consume the identifier
                        tokens.next();
                        current_scope.namespace_id = namespace_id;
                        // Get the global scope of the namespace
                        let global_scope = namespace.scope_registry.get_global_scope_id();
                        current_scope.symbol_id = global_scope;
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
