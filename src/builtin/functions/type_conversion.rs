use crate::{builtin::BuiltinRegistry, type_registry::PrimitiveType};
use cranelift::prelude::{InstBuilder, types};

pub fn register_builtins(registry: &mut BuiltinRegistry) {
    // --- INT TO FLOAT ---
    registry.register_func(
        "itof",
        &[PrimitiveType::Int],
        PrimitiveType::Float,
        Box::new(|_, builder, args| builder.ins().fcvt_from_sint(types::F32, args[0])),
    );

    // --- FLOAT TO INT ---
    registry.register_func(
        "ftoi",
        &[PrimitiveType::Float],
        PrimitiveType::Int,
        Box::new(|_, builder, args| builder.ins().fcvt_to_sint_sat(types::I32, args[0])),
    );
}
