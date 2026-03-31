//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::{ast::type_registry::PrimitiveType, builtin::BuiltinRegistry};
use kasl_ir::ir::{FloatBinOp, FloatCmp, FloatUnaryOp, InstBuilder};

pub fn register_builtins(registry: &mut BuiltinRegistry) {
    // --- BINARY OPERATORS ---

    registry.register_func(
        "fadd",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fbop(FloatBinOp::Add, args[0], args[1])),
    );

    registry.register_func(
        "fsub",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fbop(FloatBinOp::Sub, args[0], args[1])),
    );

    registry.register_func(
        "fmul",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fbop(FloatBinOp::Mul, args[0], args[1])),
    );

    registry.register_func(
        "fdiv",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fbop(FloatBinOp::Div, args[0], args[1])),
    );

    registry.register_func(
        "fmod",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| {
            let div = builder.fbop(FloatBinOp::Div, args[0], args[1]);
            let div_floor = builder.fuop(FloatUnaryOp::Floor, div);
            let floor_mul = builder.fbop(FloatBinOp::Mul, args[1], div_floor);
            builder.fbop(FloatBinOp::Sub, args[0], floor_mul)
        }),
    );

    registry.register_func(
        "fmax",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fbop(FloatBinOp::Max, args[0], args[1])),
    );

    registry.register_func(
        "fmin",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fbop(FloatBinOp::Min, args[0], args[1])),
    );

    // --- UNARY OPERATIONS ---

    registry.register_func(
        "fabs",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fuop(FloatUnaryOp::Abs, args[0])),
    );

    registry.register_func(
        "fsgn",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fuop(FloatUnaryOp::Sgn, args[0])),
    );

    registry.register_func(
        "fneg",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fuop(FloatUnaryOp::Neg, args[0])),
    );

    registry.register_func(
        "floor",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fuop(FloatUnaryOp::Floor, args[0])),
    );

    registry.register_func(
        "ceil",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fuop(FloatUnaryOp::Ceil, args[0])),
    );

    registry.register_func(
        "round",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fuop(FloatUnaryOp::Round, args[0])),
    );

    // --- COMPARISON OPERATORS ---

    registry.register_func(
        "feq",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.fcmp(FloatCmp::Eq, args[0], args[1])),
    );

    registry.register_func(
        "fne",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.fcmp(FloatCmp::Ne, args[0], args[1])),
    );

    registry.register_func(
        "fgt",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.fcmp(FloatCmp::Gt, args[0], args[1])),
    );

    registry.register_func(
        "flt",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.fcmp(FloatCmp::Lt, args[0], args[1])),
    );

    registry.register_func(
        "fge",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.fcmp(FloatCmp::Ge, args[0], args[1])),
    );

    registry.register_func(
        "fle",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.fcmp(FloatCmp::Le, args[0], args[1])),
    );
}
