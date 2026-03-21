use crate::type_registry::{PrimitiveType, ResolvedType};
use cranelift::prelude::types;
use cranelift_codegen::ir;
use cranelift_jit::JITModule;
use cranelift_module::Module;

#[derive(Copy, Clone)]
pub struct TypeConverter {
    pointer_type: ir::Type,
}

impl TypeConverter {
    pub fn new(module: &JITModule) -> Self {
        let pointer_type = module.target_config().pointer_type();
        Self { pointer_type }
    }

    pub fn convert(&self, resolved_type: &ResolvedType) -> ir::Type {
        match resolved_type {
            ResolvedType::Primitive(PrimitiveType::Int) => types::I32,
            ResolvedType::Primitive(PrimitiveType::Float) => types::F32,
            ResolvedType::Primitive(PrimitiveType::Bool) => types::I8,
            ResolvedType::Primitive(PrimitiveType::Void) => types::INVALID,
            ResolvedType::Array(_) => self.pointer_type,
            ResolvedType::Struct(_) => self.pointer_type,
        }
    }

    pub fn pointer_type(&self) -> ir::Type {
        self.pointer_type
    }
}
