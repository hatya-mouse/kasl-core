use crate::{builtin::BuiltinRegistry, type_registry::PrimitiveType};
use cranelift::prelude::{AbiParam, InstBuilder, types};
use cranelift_module::{Linkage, Module};

pub fn register_builtins(registry: &mut BuiltinRegistry) {
    // --- MATHEMATICAL FUNCTIONS ---

    registry.register_func(
        "sqrt",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|_, builder, args| builder.ins().sqrt(args[0])),
    );

    registry.register_func(
        "sin",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|module, builder, args| {
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));
            let func_id = module
                .declare_function("sin", Linkage::Import, &sig)
                .unwrap();
            let func_ref = module.declare_func_in_func(func_id, builder.func);
            let call = builder.ins().call(func_ref, &[args[0]]);
            builder.inst_results(call)[0]
        }),
    );

    registry.register_func(
        "cos",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|module, builder, args| {
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));
            let func_id = module
                .declare_function("cos", Linkage::Import, &sig)
                .unwrap();
            let func_ref = module.declare_func_in_func(func_id, builder.func);
            let call = builder.ins().call(func_ref, &[args[0]]);
            builder.inst_results(call)[0]
        }),
    );

    registry.register_func(
        "tan",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|module, builder, args| {
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));
            let func_id = module
                .declare_function("tan", Linkage::Import, &sig)
                .unwrap();
            let func_ref = module.declare_func_in_func(func_id, builder.func);
            let call = builder.ins().call(func_ref, &[args[0]]);
            builder.inst_results(call)[0]
        }),
    );

    registry.register_func(
        "asin",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|module, builder, args| {
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));
            let func_id = module
                .declare_function("asin", Linkage::Import, &sig)
                .unwrap();
            let func_ref = module.declare_func_in_func(func_id, builder.func);
            let call = builder.ins().call(func_ref, &[args[0]]);
            builder.inst_results(call)[0]
        }),
    );

    registry.register_func(
        "acos",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|module, builder, args| {
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));
            let func_id = module
                .declare_function("acos", Linkage::Import, &sig)
                .unwrap();
            let func_ref = module.declare_func_in_func(func_id, builder.func);
            let call = builder.ins().call(func_ref, &[args[0]]);
            builder.inst_results(call)[0]
        }),
    );

    registry.register_func(
        "atan",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|module, builder, args| {
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));
            let func_id = module
                .declare_function("atan", Linkage::Import, &sig)
                .unwrap();
            let func_ref = module.declare_func_in_func(func_id, builder.func);
            let call = builder.ins().call(func_ref, &[args[0]]);
            builder.inst_results(call)[0]
        }),
    );

    registry.register_func(
        "atan2",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|module, builder, args| {
            let mut sig = module.make_signature();
            sig.params
                .extend_from_slice(&[AbiParam::new(types::F32), AbiParam::new(types::F32)]);
            sig.returns.push(AbiParam::new(types::F32));
            let func_id = module
                .declare_function("atan2", Linkage::Import, &sig)
                .unwrap();
            let func_ref = module.declare_func_in_func(func_id, builder.func);
            let call = builder.ins().call(func_ref, &[args[0]]);
            builder.inst_results(call)[0]
        }),
    );

    registry.register_func(
        "fpow",
        &[PrimitiveType::Float, PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|module, builder, args| {
            let mut sig = module.make_signature();
            sig.params
                .extend_from_slice(&[AbiParam::new(types::F32), AbiParam::new(types::F32)]);
            sig.returns.push(AbiParam::new(types::F32));
            let func_id = module
                .declare_function("fpow", Linkage::Import, &sig)
                .unwrap();
            let func_ref = module.declare_func_in_func(func_id, builder.func);
            let call = builder.ins().call(func_ref, &[args[0]]);
            builder.inst_results(call)[0]
        }),
    );
}
