use kasl::{ExprToken, ExprTokenKind, ParserFuncCallArg, Range};

pub fn int_literal(value: u32) -> ExprToken {
    ExprToken {
        kind: ExprTokenKind::IntLiteral(value),
        range: Range::zero(),
    }
}

pub fn float_literal(value: f32) -> ExprToken {
    ExprToken {
        kind: ExprTokenKind::FloatLiteral(value),
        range: Range::zero(),
    }
}

pub fn bool_literal(value: bool) -> ExprToken {
    ExprToken {
        kind: ExprTokenKind::BoolLiteral(value),
        range: Range::zero(),
    }
}

pub fn operator(symbol: &str) -> ExprToken {
    ExprToken {
        kind: ExprTokenKind::Operator(symbol.to_string()),
        range: Range::zero(),
    }
}

pub fn identifier(name: &str) -> ExprToken {
    ExprToken {
        kind: ExprTokenKind::Identifier(name.to_string()),
        range: Range::zero(),
    }
}

pub fn func_call(name: &str, args: &[ParserFuncCallArg]) -> ExprToken {
    ExprToken {
        kind: ExprTokenKind::FuncCall {
            name: name.to_string(),
            args: args.to_vec(),
        },
        range: Range::zero(),
    }
}

pub fn dot() -> ExprToken {
    ExprToken {
        kind: ExprTokenKind::Dot,
        range: Range::zero(),
    }
}

pub fn parenthesized(expr: &[ExprToken]) -> ExprToken {
    ExprToken {
        kind: ExprTokenKind::Parenthesized(expr.to_vec()),
        range: Range::zero(),
    }
}

pub fn func_call_arg(label: Option<&str>, value: &[ExprToken]) -> ParserFuncCallArg {
    ParserFuncCallArg {
        label: label.map(&str::to_string),
        value: value.to_vec(),
        range: Range::zero(),
    }
}
