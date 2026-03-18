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
    CompilationData, Expr, ExprToken, NameSpaceID, ScopeID, builtin::BuiltinRegistry,
    compilation_data::ProgramContext, error::ErrorCollector,
};

pub fn resolve_expr(
    ec: &mut ErrorCollector,
    prog_ctx: &mut ProgramContext,
    comp_data: &mut CompilationData,
    builtin_registry: &BuiltinRegistry,
    current_scope_id: ScopeID,
    current_namespace: NameSpaceID,
    raw_tokens: &[ExprToken],
) -> Option<Expr> {
    // Build the expression tree
    let mut expr_builder = ExpressionBuilder::new(ec, &prog_ctx.op_ctx);
    let expr = expr_builder.build(raw_tokens)?;

    // Resolve the type of the expression
    let mut resolver = ExpressionResolver::new(
        ec,
        prog_ctx,
        comp_data,
        builtin_registry,
        current_scope_id,
        current_namespace,
    );
    resolver.resolve_recursively(expr)
}
