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
use kasl_ir::{Const, FloatBinOp, FloatUnaryOp, InstBuilder};

pub fn register_builtins(registry: &mut BuiltinRegistry) {
    // --- MATHEMATICAL FUNCTIONS ---

    registry.register_func(
        "sqrt",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fuop(FloatUnaryOp::Sqrt, args[0])),
    );

    registry.register_func(
        "fast_sin",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| {
            let x = args[0];

            let pi = builder.const_val(Const::F32(std::f32::consts::PI));
            let pi2 = builder.const_val(Const::F32(std::f32::consts::TAU));
            let x_div_pi2 = builder.fbop(FloatBinOp::Div, x, pi2);
            let x_div_pi2_floor = builder.fuop(FloatUnaryOp::Floor, x_div_pi2);
            let pi2_mul_floor = builder.fbop(FloatBinOp::Mul, pi2, x_div_pi2_floor);
            let x_mod = builder.fbop(FloatBinOp::Sub, x, pi2_mul_floor);
            let clamped_x = builder.fbop(FloatBinOp::Sub, x_mod, pi);

            let x2 = builder.fbop(FloatBinOp::Mul, clamped_x, clamped_x);
            let x3 = builder.fbop(FloatBinOp::Mul, x2, clamped_x);
            let x5 = builder.fbop(FloatBinOp::Mul, x3, x2);
            let x7 = builder.fbop(FloatBinOp::Mul, x5, x2);

            let c3 = builder.const_val(Const::F32(-1.0 / 6.0));
            let c5 = builder.const_val(Const::F32(1.0 / 120.0));
            let c7 = builder.const_val(Const::F32(-1.0 / 5040.0));

            let t3 = builder.fbop(FloatBinOp::Mul, x3, c3);
            let t5 = builder.fbop(FloatBinOp::Mul, x5, c5);
            let t7 = builder.fbop(FloatBinOp::Mul, x7, c7);

            let r = builder.fbop(FloatBinOp::Add, clamped_x, t3);
            let r = builder.fbop(FloatBinOp::Add, r, t5);
            builder.fbop(FloatBinOp::Add, r, t7)
        }),
    );

    registry.register_func(
        "fast_cos",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| {
            let x = args[0];

            let pi_half = builder.const_val(Const::F32(std::f32::consts::PI / 2.0));
            let pi2 = builder.const_val(Const::F32(std::f32::consts::TAU));
            let x_div_pi2 = builder.fbop(FloatBinOp::Div, x, pi2);
            let x_div_pi2_floor = builder.fuop(FloatUnaryOp::Floor, x_div_pi2);
            let pi2_mul_floor = builder.fbop(FloatBinOp::Mul, pi2, x_div_pi2_floor);
            let x_mod = builder.fbop(FloatBinOp::Sub, x, pi2_mul_floor);
            let clamped_x = builder.fbop(FloatBinOp::Sub, x_mod, pi_half);

            let x2 = builder.fbop(FloatBinOp::Mul, clamped_x, clamped_x);
            let x3 = builder.fbop(FloatBinOp::Mul, x2, clamped_x);
            let x5 = builder.fbop(FloatBinOp::Mul, x3, x2);
            let x7 = builder.fbop(FloatBinOp::Mul, x5, x2);

            let c3 = builder.const_val(Const::F32(-1.0 / 6.0));
            let c5 = builder.const_val(Const::F32(1.0 / 120.0));
            let c7 = builder.const_val(Const::F32(-1.0 / 5040.0));

            let t3 = builder.fbop(FloatBinOp::Mul, x3, c3);
            let t5 = builder.fbop(FloatBinOp::Mul, x5, c5);
            let t7 = builder.fbop(FloatBinOp::Mul, x7, c7);

            let r = builder.fbop(FloatBinOp::Add, clamped_x, t3);
            let r = builder.fbop(FloatBinOp::Add, r, t5);
            builder.fbop(FloatBinOp::Add, r, t7)
        }),
    );

    registry.register_func(
        "sin",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fuop(FloatUnaryOp::Sin, args[0])),
    );

    registry.register_func(
        "cos",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fuop(FloatUnaryOp::Cos, args[0])),
    );

    registry.register_func(
        "tan",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fuop(FloatUnaryOp::Tan, args[0])),
    );

    registry.register_func(
        "asin",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fuop(FloatUnaryOp::Asin, args[0])),
    );

    registry.register_func(
        "acos",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fuop(FloatUnaryOp::Acos, args[0])),
    );

    registry.register_func(
        "atan",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fuop(FloatUnaryOp::Atan, args[0])),
    );

    registry.register_func(
        "atan2",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fbop(FloatBinOp::Atan2, args[0], args[1])),
    );

    registry.register_func(
        "fpow",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.fbop(FloatBinOp::Pow, args[0], args[1])),
    );
}
