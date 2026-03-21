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
    Expr, ExprKind, Range,
    expr_engine::ExpressionResolver,
    type_registry::{PrimitiveType, ResolvedType},
};

impl ExpressionResolver<'_> {
    pub fn resolve_int_literal(&self, value: u32, range: Range) -> Option<Expr> {
        Some(Expr::new(
            ExprKind::IntLiteral(value),
            ResolvedType::Primitive(PrimitiveType::Int),
            range,
        ))
    }

    pub fn resolve_float_literal(&self, value: f32, range: Range) -> Option<Expr> {
        Some(Expr::new(
            ExprKind::FloatLiteral(value),
            ResolvedType::Primitive(PrimitiveType::Float),
            range,
        ))
    }

    pub fn resolve_bool_literal(&self, value: bool, range: Range) -> Option<Expr> {
        Some(Expr::new(
            ExprKind::BoolLiteral(value),
            ResolvedType::Primitive(PrimitiveType::Bool),
            range,
        ))
    }
}
