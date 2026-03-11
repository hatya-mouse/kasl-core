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
    ExprToken, Range, ScopeID, Statement,
    error::Ph,
    expr_engine::{LValueResolver, resolve_expr},
    scope_manager::VariableKind,
    statement_building::BlockStmtBuilder,
};

impl BlockStmtBuilder<'_> {
    pub fn build_assign(
        &mut self,
        target: &[ExprToken],
        value: &[ExprToken],
        current_scope_id: ScopeID,
        stmt_range: Range,
    ) -> Option<Statement> {
        // Resolve the target variable
        let mut l_value_resolver = LValueResolver::new(
            self.ec,
            &self.comp_state.scope_registry,
            &self.comp_state.type_registry,
            current_scope_id,
        );

        // Error will be thrown by the LValueResolver so no need to check for None
        let target_l_value = l_value_resolver.resolve_l_value(target)?;

        // Check if the LValue is a writable variable
        if let Some(target_var) = self
            .comp_state
            .scope_registry
            .get_var_by_id(&target_l_value.var_id)
            && matches!(
                target_var.var_kind,
                VariableKind::Input { .. }
                    | VariableKind::State
                    | VariableKind::LocalConst
                    | VariableKind::FuncParam
            )
        {
            self.ec
                .immutable_assignment(stmt_range, Ph::StatementCollection, &target_var.name);
            return None;
        }

        // Resolve the expression
        let resolved_value = resolve_expr(self.ec, self.comp_state, current_scope_id, value)?;

        // Check if the target and value types match
        if target_l_value.value_type != resolved_value.value_type {
            self.ec.assign_type_mismatch(
                stmt_range,
                Ph::StatementCollection,
                self.comp_state
                    .type_registry
                    .format_type(&target_l_value.value_type),
                self.comp_state
                    .type_registry
                    .format_type(&resolved_value.value_type),
            );
            return None;
        }

        Some(Statement::Assign {
            target: target_l_value,
            value: resolved_value,
        })
    }
}
