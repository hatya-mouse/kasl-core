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
use kasl_ir::{InstBuilder, IntBinOp, IntUnaryOp};

pub fn register_builtins(registry: &mut BuiltinRegistry) {
    // --- BINARY OPERATORS ---

    registry.register_func(
        "band",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.ibop(IntBinOp::BAnd, args[0], args[1])),
    );

    registry.register_func(
        "bor",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.ibop(IntBinOp::BOr, args[0], args[1])),
    );

    registry.register_func(
        "bxor",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.ibop(IntBinOp::BXor, args[0], args[1])),
    );

    registry.register_func(
        "bnand",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.ibop(IntBinOp::BNand, args[0], args[1])),
    );

    registry.register_func(
        "bnor",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.ibop(IntBinOp::BNor, args[0], args[1])),
    );

    registry.register_func(
        "bxnor",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.ibop(IntBinOp::BXnor, args[0], args[1])),
    );

    // --- UNARY OPERATORS ---

    registry.register_func(
        "bnot",
        &[PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.iuop(IntUnaryOp::BNot, args[0])),
    );
}
