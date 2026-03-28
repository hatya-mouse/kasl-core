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
use cranelift::prelude::InstBuilder;

pub fn register_builtins(registry: &mut BuiltinRegistry) {
    // --- BINARY OPERATORS ---

    registry.register_func(
        "iand",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| builder.ins().band(args[0], args[1])),
    );

    registry.register_func(
        "ior",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| builder.ins().bor(args[0], args[1])),
    );

    registry.register_func(
        "ixor",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| builder.ins().bxor(args[0], args[1])),
    );

    registry.register_func(
        "inand",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| builder.ins().band_not(args[0], args[1])),
    );

    registry.register_func(
        "inor",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| builder.ins().bor_not(args[0], args[1])),
    );

    registry.register_func(
        "ixnor",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| builder.ins().bxor_not(args[0], args[1])),
    );

    // --- BIT SHIFT ---

    registry.register_func(
        "ishl",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| builder.ins().ishl(args[0], args[1])),
    );

    registry.register_func(
        "sshr",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| builder.ins().sshr(args[0], args[1])),
    );

    registry.register_func(
        "ushr",
        &[PrimitiveType::Int, PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| builder.ins().ushr(args[0], args[1])),
    );

    // --- UNARY OPERATORS ---

    registry.register_func(
        "inot",
        &[PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|_, builder, args| builder.ins().bnot(args[0])),
    );
}
