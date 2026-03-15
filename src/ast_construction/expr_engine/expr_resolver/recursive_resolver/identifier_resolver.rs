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
    Expr, ExprKind, Range, error::Ph, expr_engine::ExpressionResolver, type_registry::ResolvedType,
};

impl ExpressionResolver<'_> {
    pub fn resolve_identifier(&mut self, name: String, range: Range) -> Option<Expr<ResolvedType>> {
        // Get the variable ID from the scope registry
        let Some(var_id) = self
            .namespace
            .scope_registry
            .lookup_var(self.current_scope, &name)
        else {
            self.ec.var_not_found(range, Ph::ExprEngine, &name);
            return None;
        };
        let var = self.namespace.scope_registry.get_var_by_id(var_id)?;

        Some(Expr::new(
            ExprKind::Identifier {
                name,
                id: Some(*var_id),
            },
            var.value_type,
            range,
        ))
    }
}
