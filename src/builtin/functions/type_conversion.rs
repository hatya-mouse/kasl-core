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
use cranelift::prelude::{InstBuilder, types};

pub fn register_builtins(registry: &mut BuiltinRegistry) {
    // --- INT TO FLOAT ---
    registry.register_func(
        "itof",
        &[PrimitiveType::Int],
        PrimitiveType::Int,
        Box::new(|builder, args| builder.ins().fcvt_from_sint(types::F32, args[0])),
    );

    // --- FLOAT TO INT ---
    registry.register_func(
        "ftoi",
        &[PrimitiveType::Float],
        PrimitiveType::Int,
        Box::new(|builder, args| builder.ins().fcvt_to_sint_sat(types::I32, args[0])),
    );
}
