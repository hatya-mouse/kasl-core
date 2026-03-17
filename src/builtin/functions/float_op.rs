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

use crate::{builtin::BuiltinRegistry, type_registry::PrimitiveType};
use cranelift::prelude::{FloatCC, InstBuilder, types};

pub fn register_builtins(registry: &mut BuiltinRegistry) {
    // --- BINARY OPERATORS ---

    registry.register_func(
        "fadd",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.ins().fadd(args[0], args[1])),
    );

    registry.register_func(
        "fsub",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.ins().fsub(args[0], args[1])),
    );

    registry.register_func(
        "fmul",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.ins().fmul(args[0], args[1])),
    );

    registry.register_func(
        "fdiv",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.ins().fdiv(args[0], args[1])),
    );

    registry.register_func(
        "fmod",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| {
            let div = builder.ins().fdiv(args[0], args[1]);
            let div_floor = builder.ins().floor(div);
            let floor_mul = builder.ins().fmul(args[1], div_floor);
            builder.ins().fsub(args[0], floor_mul)
        }),
    );

    registry.register_func(
        "fmax",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| {
            let is_lhs_greater = builder.ins().fcmp(FloatCC::GreaterThan, args[0], args[1]);
            builder.ins().select(is_lhs_greater, args[0], args[1])
        }),
    );

    registry.register_func(
        "fmin",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| {
            let is_lhs_lesser = builder.ins().fcmp(FloatCC::LessThan, args[0], args[1]);
            builder.ins().select(is_lhs_lesser, args[0], args[1])
        }),
    );

    // --- UNARY OPERATIONS ---

    registry.register_func(
        "fabs",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.ins().fabs(args[0])),
    );

    registry.register_func(
        "fsgn",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| {
            let one = builder.ins().f32const(1.0);
            let zero = builder.ins().f32const(0.0);
            let minus_one = builder.ins().f32const(-1.0);

            let is_positive = builder.ins().fcmp(FloatCC::GreaterThan, args[0], zero);
            let is_negative = builder.ins().fcmp(FloatCC::LessThan, args[0], zero);

            let pos_val = builder.ins().select(is_positive, one, zero);
            builder.ins().select(is_negative, minus_one, pos_val)
        }),
    );

    registry.register_func(
        "floor",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.ins().floor(args[0])),
    );

    registry.register_func(
        "ceil",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.ins().ceil(args[0])),
    );

    registry.register_func(
        "round",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|builder, args| builder.ins().nearest(args[0])),
    );

    // --- COMPARISON OPERATORS ---

    registry.register_func(
        "feq",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Bool,
        Box::new(|builder, args| {
            let result = builder.ins().fcmp(FloatCC::Equal, args[0], args[1]);
            builder.ins().ireduce(types::I8, result)
        }),
    );

    registry.register_func(
        "fne",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Bool,
        Box::new(|builder, args| {
            let result = builder.ins().fcmp(FloatCC::NotEqual, args[0], args[1]);
            builder.ins().ireduce(types::I8, result)
        }),
    );

    registry.register_func(
        "fgt",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Bool,
        Box::new(|builder, args| {
            let result = builder.ins().fcmp(FloatCC::GreaterThan, args[0], args[1]);
            builder.ins().ireduce(types::I8, result)
        }),
    );

    registry.register_func(
        "flt",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Bool,
        Box::new(|builder, args| {
            let result = builder.ins().fcmp(FloatCC::LessThan, args[0], args[1]);
            builder.ins().ireduce(types::I8, result)
        }),
    );

    registry.register_func(
        "fge",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Bool,
        Box::new(|builder, args| {
            let result = builder
                .ins()
                .fcmp(FloatCC::GreaterThanOrEqual, args[0], args[1]);
            builder.ins().ireduce(types::I8, result)
        }),
    );

    registry.register_func(
        "fle",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Bool,
        Box::new(|builder, args| {
            let result = builder
                .ins()
                .fcmp(FloatCC::LessThanOrEqual, args[0], args[1]);
            builder.ins().ireduce(types::I8, result)
        }),
    );
}
