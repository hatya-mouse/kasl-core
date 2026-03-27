use crate::{builtin::BuiltinRegistry, type_registry::PrimitiveType};
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
