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
