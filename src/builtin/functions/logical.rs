use crate::{builtin::BuiltinRegistry, type_registry::PrimitiveType};
use cranelift::prelude::InstBuilder;

pub fn register_builtins(registry: &mut BuiltinRegistry) {
    // --- BINARY OPERATORS ---

    registry.register_func(
        "band",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|_, builder, args| builder.ins().band(args[0], args[1])),
    );

    registry.register_func(
        "bor",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|_, builder, args| builder.ins().bor(args[0], args[1])),
    );

    registry.register_func(
        "bxor",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|_, builder, args| builder.ins().bxor(args[0], args[1])),
    );

    registry.register_func(
        "bnand",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|_, builder, args| builder.ins().band_not(args[0], args[1])),
    );

    registry.register_func(
        "bnor",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|_, builder, args| builder.ins().bor_not(args[0], args[1])),
    );

    registry.register_func(
        "bxnor",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|_, builder, args| builder.ins().bxor_not(args[0], args[1])),
    );

    // --- UNARY OPERATORS ---

    registry.register_func(
        "bnot",
        &[PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|_, builder, args| builder.ins().bnot(args[0])),
    );
}
