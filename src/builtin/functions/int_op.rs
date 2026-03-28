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
use cranelift::prelude::{InstBuilder, IntCC, types};

pub fn register_builtins(registry: &mut BuiltinRegistry) {
    // --- BINARY OPERATORS ---

    registry.register_func(
        "iadd",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| builder.ins().iadd(args[0], args[1])),
    );

    registry.register_func(
        "isub",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| builder.ins().isub(args[0], args[1])),
    );

    registry.register_func(
        "imul",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| builder.ins().imul(args[0], args[1])),
    );

    registry.register_func(
        "idiv",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| {
            let diviend = args[0];
            let divisor = args[1];

            let is_divisor_zero = builder.ins().icmp_imm(IntCC::Equal, divisor, 0);
            let one = builder.ins().iconst(types::I32, 1);
            let safe_divisor = builder.ins().select(is_divisor_zero, one, divisor);

            let quotient = builder.ins().sdiv(diviend, safe_divisor);
            let zero = builder.ins().iconst(types::I32, 0);
            builder.ins().select(is_divisor_zero, zero, quotient)
        }),
    );

    registry.register_func(
        "imod",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| {
            let diviend = args[0];
            let divisor = args[1];

            let is_divisor_zero = builder.ins().icmp_imm(IntCC::Equal, divisor, 0);
            let one = builder.ins().iconst(types::I32, 1);
            let safe_divisor = builder.ins().select(is_divisor_zero, one, divisor);

            let rem = builder.ins().srem(diviend, safe_divisor);
            let zero = builder.ins().iconst(types::I32, 0);
            builder.ins().select(is_divisor_zero, zero, rem)
        }),
    );

    // --- CLAMP FUNCTIONS ---

    registry.register_func(
        "imax",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| {
            let is_lhs_greater = builder
                .ins()
                .icmp(IntCC::SignedGreaterThan, args[0], args[1]);
            builder.ins().select(is_lhs_greater, args[0], args[1])
        }),
    );

    registry.register_func(
        "imin",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| {
            let is_lhs_lesser = builder.ins().icmp(IntCC::SignedLessThan, args[0], args[1]);
            builder.ins().select(is_lhs_lesser, args[0], args[1])
        }),
    );

    // --- UNARY OPERATIONS ---

    registry.register_func(
        "iabs",
        &[PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| builder.ins().iabs(args[0])),
    );

    registry.register_func(
        "isgn",
        &[PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| {
            let is_positive = builder.ins().icmp_imm(IntCC::SignedGreaterThan, args[0], 0);
            let is_negative = builder.ins().icmp_imm(IntCC::SignedLessThan, args[0], 0);

            let one = builder.ins().iconst(types::I32, 1);
            let zero = builder.ins().iconst(types::I32, 0);
            let minus_one = builder.ins().iconst(types::I32, -1);
            let pos_val = builder.ins().select(is_positive, one, zero);
            builder.ins().select(is_negative, minus_one, pos_val)
        }),
    );

    registry.register_func(
        "ineg",
        &[PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| builder.ins().ineg(args[0])),
    );

    // --- COMPARISON OPERATORS ---

    registry.register_func(
        "ieq",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Bool,
        Box::new(|_, builder, args| builder.ins().icmp(IntCC::Equal, args[0], args[1])),
    );

    registry.register_func(
        "ine",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Bool,
        Box::new(|_, builder, args| builder.ins().icmp(IntCC::NotEqual, args[0], args[1])),
    );

    registry.register_func(
        "igt",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Bool,
        Box::new(|_, builder, args| {
            builder
                .ins()
                .icmp(IntCC::SignedGreaterThan, args[0], args[1])
        }),
    );

    registry.register_func(
        "ilt",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Bool,
        Box::new(|_, builder, args| builder.ins().icmp(IntCC::SignedLessThan, args[0], args[1])),
    );

    registry.register_func(
        "ige",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Bool,
        Box::new(|_, builder, args| {
            builder
                .ins()
                .icmp(IntCC::SignedGreaterThanOrEqual, args[0], args[1])
        }),
    );

    registry.register_func(
        "ile",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Bool,
        Box::new(|_, builder, args| {
            builder
                .ins()
                .icmp(IntCC::SignedLessThanOrEqual, args[0], args[1])
        }),
    );
}
