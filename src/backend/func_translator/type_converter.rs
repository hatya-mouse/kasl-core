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
            ResolvedType::Struct(_) => self.pointer_type,
        }
    }
}
