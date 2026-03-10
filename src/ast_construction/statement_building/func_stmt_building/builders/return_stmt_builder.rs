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
    ExprToken, Range, ScopeID, Statement, error::Ph, expr_engine::resolve_expr,
    statement_building::FuncStmtBuilder,
};

impl FuncStmtBuilder<'_> {
    pub fn build_return_stmt(
        &mut self,
        value: Option<&Vec<ExprToken>>,
        current_scope_id: ScopeID,
        decl_range: Range,
    ) -> Option<Statement> {
        if let Some(expected_return_type) = self.expected_return_type {
            if let Some(value) = value {
                // Resolve the expression
                let resolved_value =
                    resolve_expr(self.ec, self.compilation_state, current_scope_id, value)?;

                // Check if the return type matches the expected return type
                // If the self.expected_return_type is None, resolved_value should be None as well
                if resolved_value.value_type != expected_return_type {
                    self.ec.return_type_mismatch(
                        decl_range,
                        Ph::StatementCollection,
                        self.compilation_state
                            .type_registry
                            .format_type(&expected_return_type),
                        self.compilation_state
                            .type_registry
                            .format_type(&resolved_value.value_type),
                    );
                    return None;
                }

                Some(Statement::Return {
                    value: Some(resolved_value),
                })
            } else {
                self.ec.return_without_value_for_return_func(
                    decl_range,
                    Ph::StatementCollection,
                    self.compilation_state
                        .type_registry
                        .format_type(&expected_return_type),
                );
                None
            }
        } else if value.is_some() {
            self.ec
                .return_value_for_no_return_func(decl_range, Ph::StatementCollection);
            None
        } else {
            Some(Statement::Return { value: None })
        }
    }
}
