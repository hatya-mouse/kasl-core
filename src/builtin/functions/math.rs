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
        Box::new(|_, builder, args| {
            let x = args[0];

            let pi = builder.ins().f32const(std::f32::consts::PI);
            let pi2 = builder.ins().f32const(std::f32::consts::TAU);
            let x_div_pi2 = builder.ins().fdiv(x, pi2);
            let x_div_pi2_floor = builder.ins().floor(x_div_pi2);
            let pi2_mul_floor = builder.ins().fmul(pi2, x_div_pi2_floor);
            let x_mod = builder.ins().fsub(x, pi2_mul_floor);
            let clamped_x = builder.ins().fsub(x_mod, pi);

            let x2 = builder.ins().fmul(clamped_x, clamped_x);
            let x3 = builder.ins().fmul(x2, clamped_x);
            let x5 = builder.ins().fmul(x3, x2);
            let x7 = builder.ins().fmul(x5, x2);
            let x9 = builder.ins().fmul(x7, x2);
            let x11 = builder.ins().fmul(x9, x2);

            let c3 = builder.ins().f32const(-1.0 / 6.0);
            let c5 = builder.ins().f32const(1.0 / 120.0);
            let c7 = builder.ins().f32const(-1.0 / 5040.0);
            let c9 = builder.ins().f32const(1.0 / 362880.0);
            let c11 = builder.ins().f32const(-1.0 / 39916800.0);

            let t3 = builder.ins().fmul(x3, c3);
            let t5 = builder.ins().fmul(x5, c5);
            let t7 = builder.ins().fmul(x7, c7);
            let t9 = builder.ins().fmul(x9, c9);
            let t11 = builder.ins().fmul(x11, c11);

            let r = builder.ins().fadd(clamped_x, t3);
            let r = builder.ins().fadd(r, t5);
            let r = builder.ins().fadd(r, t7);
            let r = builder.ins().fadd(r, t9);
            builder.ins().fadd(r, t11)
        }),
    );

    registry.register_func(
        "cos",
        &[PrimitiveType::Float],
        PrimitiveType::Float,
        Box::new(|_, builder, args| {
            let x = args[0];

            let pi_half = builder.ins().f32const(std::f32::consts::PI / 2.0);
            let pi2 = builder.ins().f32const(std::f32::consts::TAU);
            let x_div_pi2 = builder.ins().fdiv(x, pi2);
            let x_div_pi2_floor = builder.ins().floor(x_div_pi2);
            let pi2_mul_floor = builder.ins().fmul(pi2, x_div_pi2_floor);
            let x_mod = builder.ins().fsub(x, pi2_mul_floor);
            let clamped_x = builder.ins().fsub(x_mod, pi_half);

            let x2 = builder.ins().fmul(clamped_x, clamped_x);
            let x3 = builder.ins().fmul(x2, clamped_x);
            let x5 = builder.ins().fmul(x3, x2);
            let x7 = builder.ins().fmul(x5, x2);
            let x9 = builder.ins().fmul(x7, x2);
            let x11 = builder.ins().fmul(x9, x2);

            let c3 = builder.ins().f32const(-1.0 / 6.0);
            let c5 = builder.ins().f32const(1.0 / 120.0);
            let c7 = builder.ins().f32const(-1.0 / 5040.0);
            let c9 = builder.ins().f32const(1.0 / 362880.0);
            let c11 = builder.ins().f32const(-1.0 / 39916800.0);

            let t3 = builder.ins().fmul(x3, c3);
            let t5 = builder.ins().fmul(x5, c5);
            let t7 = builder.ins().fmul(x7, c7);
            let t9 = builder.ins().fmul(x9, c9);
            let t11 = builder.ins().fmul(x11, c11);

            let r = builder.ins().fadd(clamped_x, t3);
            let r = builder.ins().fadd(r, t5);
            let r = builder.ins().fadd(r, t7);
            let r = builder.ins().fadd(r, t9);
            builder.ins().fadd(r, t11)
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
