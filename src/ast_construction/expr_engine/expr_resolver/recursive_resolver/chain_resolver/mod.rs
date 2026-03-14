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

mod field_access_resolver;
mod member_access_resolver;
mod member_func_resolver;

use crate::{
    Expr, ExprKind, Range, expr_engine::ExpressionResolver, symbol_table::MemberAccess,
    type_registry::ResolvedType,
};

impl ExpressionResolver<'_> {
    pub fn resolve_chain(
        &mut self,
        lhs: Expr<()>,
        access: MemberAccess,
        range: Range,
    ) -> Option<Expr<ResolvedType>> {
        // Resolve the LHS expression
        let resolved_lhs = self.resolve_recursively(lhs)?;

        // Resolve the access expression
        let (resolved_access, value_type) =
            self.resolve_member_access(&resolved_lhs.value_type, access, range)?;

        Some(Expr::new(
            ExprKind::Chain {
                lhs: Box::new(resolved_lhs),
                access: resolved_access,
            },
            value_type,
            range,
        ))
    }
}
