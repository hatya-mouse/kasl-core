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
    Expr, ExprKind, expr_engine::ExpressionResolver, symbol_path, symbol_table::ResolvedChainLHS,
};

impl ExpressionResolver<'_> {
    pub fn resolve_chain_lhs(&mut self, raw_expr: Expr<()>) -> Option<ResolvedChainLHS> {
        // Chain LHS can be a type because there are static functions
        if let ExprKind::Identifier { name, .. } = &raw_expr.kind {
            if name == "Builtin" {
                return Some(ResolvedChainLHS::Builtin);
            } else if let Some(resolved_type) = self
                .namespace
                .type_registry
                .resolve_type_path(&symbol_path![name])
            {
                return Some(ResolvedChainLHS::Type(resolved_type));
            }
        }

        self.resolve_recursively(raw_expr)
            .map(ResolvedChainLHS::Expr)
    }
}
