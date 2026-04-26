//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::ast_nodes::type_registry::{PrimitiveType, ResolvedType};
use kasl_ir::IRType;

pub(super) fn convert_type(resolved_type: &ResolvedType) -> IRType {
    match resolved_type {
        ResolvedType::Primitive(PrimitiveType::Int) => IRType::I32,
        ResolvedType::Primitive(PrimitiveType::Float) => IRType::F32,
        ResolvedType::Primitive(PrimitiveType::Bool) => IRType::I8,
        ResolvedType::Primitive(PrimitiveType::Void) => IRType::Void,
        ResolvedType::Array(_) => IRType::Ptr,
        ResolvedType::Struct(_) => IRType::Ptr,
    }
}
