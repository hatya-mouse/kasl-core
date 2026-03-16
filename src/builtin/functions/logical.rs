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
use cranelift::prelude::InstBuilder;

pub fn register_builtins(registry: &mut BuiltinRegistry) {
    // --- BINARY OPERATORS ---

    registry.register_func(
        "and",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.ins().band(args[0], args[1])),
    );

    registry.register_func(
        "or",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.ins().bor(args[0], args[1])),
    );

    registry.register_func(
        "xor",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.ins().bxor(args[0], args[1])),
    );

    registry.register_func(
        "nand",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.ins().band_not(args[0], args[1])),
    );

    registry.register_func(
        "nor",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.ins().bor_not(args[0], args[1])),
    );

    registry.register_func(
        "xnor",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.ins().bxor_not(args[0], args[1])),
    );

    // --- UNARY OPERATORS ---

    registry.register_func(
        "not",
        &[PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|builder, args| builder.ins().bnot(args[0])),
    );
}
