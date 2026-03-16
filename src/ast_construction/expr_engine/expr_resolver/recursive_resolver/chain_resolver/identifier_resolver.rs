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
    Expr, ExprKind, Range, ScopeID,
    error::Ph,
    expr_engine::ExpressionResolver,
    namespace_registry::{NameSpacePair, NameSpaceVarGetter},
};

impl ExpressionResolver<'_> {
    pub fn resolve_identifier(
        &mut self,
        target_scope: NameSpacePair<ScopeID>,
        name: &str,
        range: Range,
    ) -> Option<Expr> {
        // Look up the variable in the target scope
        let Some(var_id) = self.namespace_registry.get_var_id(&target_scope, name) else {
            self.ec.var_not_found(range, Ph::ExprEngine, name);
            return None;
        };

        // Get a reference to the variable
        let scope_var = self.namespace_registry.get_var(&var_id)?;

        Some(Expr::new(
            ExprKind::Identifier { id: var_id },
            scope_var.value_type,
            range,
        ))
    }
}
