use crate::type_registry::ResolvedType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct ArrayDecl {
    item_type: ResolvedType,
    count: u32,
}

impl ArrayDecl {
    pub fn new(item_type: ResolvedType, count: u32) -> Self {
        Self { item_type, count }
    }

    pub fn item_type(&self) -> &ResolvedType {
        &self.item_type
    }

    pub fn count(&self) -> &u32 {
        &self.count
    }
}
