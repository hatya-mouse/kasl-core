use crate::{StructID, namespace_registry::ArrayID, type_registry::PrimitiveType};

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
