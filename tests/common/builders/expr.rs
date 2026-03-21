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
