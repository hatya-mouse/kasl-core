mod builtin_func;
mod functions;

use crate::type_registry::{PrimitiveType, ResolvedType};
pub use builtin_func::{BuiltinFunc, BuiltinFuncID, BuiltinFuncTranslator};
use cranelift_jit::JITBuilder;
use std::collections::HashMap;

pub struct BuiltinRegistry {
    functions: HashMap<BuiltinFuncID, BuiltinFunc>,
    name_to_id: HashMap<String, BuiltinFuncID>,
    next_builtin_func_id: usize,
}

impl Default for BuiltinRegistry {
    fn default() -> Self {
        let mut registry = Self::new();

        functions::bool_op::register_builtins(&mut registry);
        functions::float_op::register_builtins(&mut registry);
        functions::int_op::register_builtins(&mut registry);
        functions::logical::register_builtins(&mut registry);
        functions::math::register_builtins(&mut registry);
        functions::type_conversion::register_builtins(&mut registry);

        registry
    }
}

impl BuiltinRegistry {
    fn new() -> Self {
        Self {
            functions: HashMap::new(),
            name_to_id: HashMap::new(),
            next_builtin_func_id: 0,
        }
    }

    fn generate_id(&mut self) -> BuiltinFuncID {
        let id = BuiltinFuncID::new(self.next_builtin_func_id);
        self.next_builtin_func_id += 1;
        id
    }

    pub(crate) fn register_symbols(builder: &mut JITBuilder) {
        // --- TRIGONOMETRIC FUNCTIONS ---
        builder.symbol("sin", f32::sin as *const u8);
        builder.symbol("cos", f32::cos as *const u8);
        builder.symbol("tan", f32::tan as *const u8);
        builder.symbol("asin", f32::asin as *const u8);
        builder.symbol("acos", f32::acos as *const u8);
        builder.symbol("atan", f32::atan as *const u8);
        builder.symbol("atan2", f32::atan2 as *const u8);

        builder.symbol("fpow", f32::powf as *const u8);
    }

    pub(in crate::builtin) fn register_func(
        &mut self,
        name: &'static str,
        params: &[PrimitiveType],
        return_type: PrimitiveType,
        translator: BuiltinFuncTranslator,
    ) {
        let func_id = self.generate_id();
        let func = BuiltinFunc {
            name,
            params: params
                .iter()
                .map(|ty| ResolvedType::Primitive(*ty))
                .collect(),
            return_type: ResolvedType::Primitive(return_type),
            translator,
        };
        self.functions.insert(func_id, func);
        self.name_to_id.insert(name.to_string(), func_id);
    }

    pub fn get_id_by_name(&self, name: &str) -> Option<&BuiltinFuncID> {
        self.name_to_id.get(name)
    }

    pub fn get_func_by_id(&self, id: &BuiltinFuncID) -> Option<&BuiltinFunc> {
        self.functions.get(id)
    }
}
