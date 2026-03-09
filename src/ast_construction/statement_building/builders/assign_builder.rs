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
    ExprToken, Range, ScopeID, Statement, SymbolPath, expr_engine::resolve_expr,
    statement_building::StatementBuilder,
};

impl StatementBuilder<'_> {
    pub fn build_assign(
        &mut self,
        target: SymbolPath,
        value: &[ExprToken],
        current_scope_id: ScopeID,
        stmt_range: Range,
    ) -> Option<Statement> {
        // Resolve the target variable

        // Resolve the expression
        let resolved_value = resolve_expr(
            self.ec,
            self.op_ctx,
            self.func_ctx,
            self.scope_registry,
            self.type_registry,
            current_scope_id,
            value,
        )?;
    }
}
