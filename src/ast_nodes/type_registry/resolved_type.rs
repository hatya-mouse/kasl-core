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

use crate::ast_nodes::{StructID, namespace_registry::ArrayID, type_registry::PrimitiveType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ResolvedType {
    Primitive(PrimitiveType),
    Array(ArrayID),
    Struct(StructID),
}

impl PartialEq<ResolvedType> for &ResolvedType {
    fn eq(&self, other: &ResolvedType) -> bool {
        match (self, other) {
            (ResolvedType::Primitive(ty1), ResolvedType::Primitive(ty2)) => ty1 == ty2,
            (ResolvedType::Array(id1), ResolvedType::Array(id2)) => id1 == id2,
            (ResolvedType::Struct(id1), ResolvedType::Struct(id2)) => id1 == id2,
            _ => false,
        }
    }
}

impl ResolvedType {
    pub fn is_void(&self) -> bool {
        self == ResolvedType::Primitive(PrimitiveType::Void)
    }
}
