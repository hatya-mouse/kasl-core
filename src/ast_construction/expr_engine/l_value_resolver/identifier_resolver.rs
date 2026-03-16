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

use crate::{Range, ScopeID, error::Ph, expr_engine::LValueResolver, symbol_table::LValue};

impl LValueResolver<'_> {
    pub fn resolve_identifier(
        &mut self,
        target_scope: ScopeID,
        name: &str,
        range: Range,
    ) -> Option<LValue> {
        // Look up the variable ID in the current scope
        let Some(var_id) = self.prog_ctx.scope_registry.get_var_id(target_scope, name) else {
            self.ec.var_not_found(range, Ph::ExprEngine, name);
            return None;
        };

        // Get the variable's type
        let var = self.prog_ctx.scope_registry.get_var(&var_id)?;

        // Create and return a LValue
        Some(LValue {
            var_id,
            offset: 0,
            value_type: var.value_type,
            is_field: false,
        })
    }
}
