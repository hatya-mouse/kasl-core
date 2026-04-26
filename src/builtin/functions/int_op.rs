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

use crate::{ast_nodes::type_registry::PrimitiveType, builtin::BuiltinRegistry};
use kasl_ir::{Const, InstBuilder, IntBinOp, IntCmp, IntUnaryOp};

pub fn register_builtins(registry: &mut BuiltinRegistry) {
    // --- BINARY OPERATORS ---

    registry.register_func(
        "iadd",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|builder, args| builder.ibop(IntBinOp::Add, args[0], args[1])),
    );

    registry.register_func(
        "isub",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|builder, args| builder.ibop(IntBinOp::Sub, args[0], args[1])),
    );

    registry.register_func(
        "imul",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|builder, args| builder.ibop(IntBinOp::Mul, args[0], args[1])),
    );

    registry.register_func(
        "idiv",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|builder, args| {
            let diviend = args[0];
            let divisor = args[1];

            let is_divisor_zero = builder.icmp_imm(IntCmp::Eq, divisor, 0);
            let one = builder.const_val(Const::I32(1));
            let safe_divisor = builder.select(is_divisor_zero, one, divisor);

            builder.ibop(IntBinOp::Div, diviend, safe_divisor)
        }),
    );

    registry.register_func(
        "imod",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|builder, args| {
            let diviend = args[0];
            let divisor = args[1];

            let is_divisor_zero = builder.icmp_imm(IntCmp::Eq, divisor, 0);
            let one = builder.const_val(Const::I32(1));
            let safe_divisor = builder.select(is_divisor_zero, one, divisor);

            let rem = builder.ibop(IntBinOp::SRem, diviend, safe_divisor);
            let zero = builder.const_val(Const::I32(0));
            builder.select(is_divisor_zero, zero, rem)
        }),
    );

    // --- CLAMP FUNCTIONS ---

    registry.register_func(
        "imax",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|builder, args| builder.ibop(IntBinOp::Max, args[0], args[1])),
    );

    registry.register_func(
        "imin",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|builder, args| builder.ibop(IntBinOp::Min, args[0], args[1])),
    );

    // --- UNARY OPERATIONS ---

    registry.register_func(
        "iabs",
        &[PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|builder, args| builder.iuop(IntUnaryOp::Abs, args[0])),
    );

    registry.register_func(
        "isgn",
        &[PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|builder, args| builder.iuop(IntUnaryOp::Sgn, args[0])),
    );

    registry.register_func(
        "ineg",
        &[PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|builder, args| builder.iuop(IntUnaryOp::Neg, args[0])),
    );

    // --- COMPARISON OPERATORS ---

    registry.register_func(
        "ieq",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.icmp(IntCmp::Eq, args[0], args[1])),
    );

    registry.register_func(
        "ine",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.icmp(IntCmp::Ne, args[0], args[1])),
    );

    registry.register_func(
        "igt",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.icmp(IntCmp::Sgt, args[0], args[1])),
    );

    registry.register_func(
        "ilt",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.icmp(IntCmp::Slt, args[0], args[1])),
    );

    registry.register_func(
        "ige",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.icmp(IntCmp::Sge, args[0], args[1])),
    );

    registry.register_func(
        "ile",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.icmp(IntCmp::Sle, args[0], args[1])),
    );
}
