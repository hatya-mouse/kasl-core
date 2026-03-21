use crate::{builtin::BuiltinRegistry, type_registry::PrimitiveType};
use cranelift::prelude::{InstBuilder, IntCC};

pub fn register_builtins(registry: &mut BuiltinRegistry) {
    // --- COMPARISON OPERATORS ---

    registry.register_func(
        "beq",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|_, builder, args| builder.ins().icmp(IntCC::Equal, args[0], args[1])),
    );

    registry.register_func(
        "bne",
        &[PrimitiveType::Bool, PrimitiveType::Bool],
        PrimitiveType::Bool,
        Box::new(|_, builder, args| builder.ins().icmp(IntCC::NotEqual, args[0], args[1])),
    );
}
