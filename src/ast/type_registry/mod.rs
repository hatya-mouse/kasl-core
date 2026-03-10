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

mod primitive_type;
mod resolved_type;
mod struct_decl;
mod struct_field;

pub use primitive_type::PrimitiveType;
pub use resolved_type::ResolvedType;
pub use struct_decl::StructDecl;
pub use struct_field::StructField;

use crate::{StructID, SymbolPath};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Default)]
pub struct TypeRegistry {
    pub structs: HashMap<StructID, StructDecl>,
    pub path_to_id: HashMap<SymbolPath, StructID>,
}

impl TypeRegistry {
    pub fn resolve_type_path(&self, type_path: &SymbolPath) -> Option<ResolvedType> {
        if type_path.len() == 1
            && let Ok(primitive_type) = PrimitiveType::from_str(&type_path.last().unwrap().symbol)
        {
            return Some(ResolvedType::Primitive(primitive_type));
        }
        let id = self.get_struct_id_by_path(type_path)?;
        Some(ResolvedType::Struct(id))
    }

    pub fn get_struct_id_by_path(&self, type_path: &SymbolPath) -> Option<StructID> {
        self.path_to_id.get(type_path).copied()
    }

    pub fn get_type_size(&self, type_id: &ResolvedType) -> i32 {
        match type_id {
            ResolvedType::Primitive(ty) => ty.size(),
            ResolvedType::Struct(id) => self.structs[id].total_size,
        }
    }

    pub fn get_type_alignment(&self, type_id: &ResolvedType) -> i32 {
        match type_id {
            ResolvedType::Primitive(ty) => ty.alignment(),
            ResolvedType::Struct(id) => self.structs[id].alignment,
        }
    }

    pub fn register_struct(&mut self, struct_decl: StructDecl, path: SymbolPath, id: StructID) {
        self.structs.insert(id, struct_decl);
        self.path_to_id.insert(path, id);
    }

    pub fn get_struct(&self, id: &StructID) -> Option<&StructDecl> {
        self.structs.get(id)
    }

    pub fn has_struct(&self, path: &SymbolPath) -> bool {
        self.path_to_id.contains_key(path)
    }

    pub fn format_type(&self, ty: &ResolvedType) -> String {
        match ty {
            ResolvedType::Primitive(ty) => ty.to_string(),
            ResolvedType::Struct(id) => self
                .get_struct(id)
                .map(|s| s.name.clone())
                .unwrap_or(format!("struct(ID: {})", id)),
        }
    }
}
