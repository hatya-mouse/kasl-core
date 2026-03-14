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

mod expr_builder;
mod expr_resolver;
mod l_value_resolver;

pub use expr_builder::ExpressionBuilder;
pub use expr_resolver::ExpressionResolver;
pub use l_value_resolver::LValueResolver;

use crate::{
    CompilationState, Expr, ExprToken, ScopeID, error::ErrorCollector, scope_manager::ScopeGraph,
    type_registry::ResolvedType,
};

pub fn resolve_expr(
    ec: &mut ErrorCollector,
    comp_state: &CompilationState,
    scope_graph: &mut ScopeGraph,
    current_scope_id: ScopeID,
    raw_tokens: &[ExprToken],
) -> Option<Expr<ResolvedType>> {
    // Build the expression tree
    let mut expr_builder = ExpressionBuilder::new(ec, &comp_state.op_ctx);
    let expr = expr_builder.build(raw_tokens)?;

    // Resolve the type of the expression
    let mut resolver = ExpressionResolver::new(ec, comp_state, scope_graph, current_scope_id);
    resolver.resolve_recursively(expr)
}
