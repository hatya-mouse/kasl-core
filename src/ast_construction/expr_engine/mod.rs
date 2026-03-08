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

pub use expr_builder::ExpressionBuilder;
pub use expr_resolver::ExpressionResolver;

use crate::{
    Expr, ExprToken, OperatorContext, ScopeID, ScopeRegistry,
    error::ErrorCollector,
    symbol_table::FunctionContext,
    type_registry::{ResolvedType, TypeRegistry},
};

pub fn resolve_expr(
    ec: &mut ErrorCollector,
    op_ctx: &OperatorContext,
    func_ctx: &FunctionContext,
    scope_registry: &ScopeRegistry,
    type_registry: &TypeRegistry,
    current_scope: ScopeID,
    raw_tokens: Vec<ExprToken>,
) -> Option<Expr<ResolvedType>> {
    // Build the expression tree
    let mut expr_builder = ExpressionBuilder::new(ec, op_ctx);
    let expr = expr_builder.build(raw_tokens)?;

    // Resolve the type of the expression
    let mut resolver = ExpressionResolver::new(
        ec,
        op_ctx,
        func_ctx,
        scope_registry,
        type_registry,
        current_scope,
    );
    resolver.resolve_recursively(expr)
}
