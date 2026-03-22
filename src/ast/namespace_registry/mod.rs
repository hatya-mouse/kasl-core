mod import_path;
mod namespace;
mod symbol_id;
mod symbol_path;

pub use import_path::ImportPath;
pub use namespace::NameSpace;
pub use symbol_id::{
    ArrayID, FunctionID, NameSpaceID, OperatorID, ParserStmtID, StructID, VariableID,
};
pub use symbol_path::{SymbolPath, SymbolPathComponent};

use std::collections::HashMap;

/// Stores a id-namespace pair of all namespaces in compilation.
/// Should only exist one instance per compilation.
#[derive(Debug)]
pub struct NameSpaceRegistry {
    namespaces: HashMap<NameSpaceID, NameSpace>,
    root_namespace_id: NameSpaceID,
    next_namespace_id: usize,
}

impl Default for NameSpaceRegistry {
    /// Creates a new `NameSpaceRegistry` with the root namespace registered.
    fn default() -> Self {
        let mut registry = Self {
            namespaces: HashMap::new(),
            root_namespace_id: NameSpaceID(0),
            next_namespace_id: 1,
        };
        // Register the root namespace
        registry
            .namespaces
            .insert(registry.root_namespace_id, NameSpace::default());
        registry
    }
}

impl NameSpaceRegistry {
    pub fn generate_namespace_id(&mut self) -> NameSpaceID {
        let id = NameSpaceID(self.next_namespace_id);
        self.next_namespace_id += 1;
        id
    }

    // --- GETTER FUNCTIONS ---

    pub fn get_root_namespace_id(&self) -> NameSpaceID {
        self.root_namespace_id
    }

    pub fn get_namespace_by_id(&self, id: &NameSpaceID) -> Option<&NameSpace> {
        self.namespaces.get(id)
    }

    // --- REGISTRATION ---

    pub fn register_namespace(&mut self, name: String, parent: Option<NameSpaceID>) -> NameSpaceID {
        let id = self.generate_namespace_id();
        let namespace = NameSpace::new(id);
        self.namespaces.insert(id, namespace);

        // Register the new namespace as a child of the parent namespace
        if let Some(parent) = parent
            && let Some(parent) = self.namespaces.get_mut(&parent)
        {
            parent.add_child(name, id);
        }
        id
    }

    // --- NAMESPACE RESOLUTION ---

    /// Resolves a namespace from a first few path components and returns the corresponding namespace and the remaining path components.
    pub fn resolve_namespace_from_path(
        &self,
        current_namespace: NameSpaceID,
        path: SymbolPath,
    ) -> (NameSpaceID, SymbolPath) {
        let mut path_iter = path.into_iter().peekable();
        let mut current_namespace_id = current_namespace;
        while let Some(name) = path_iter.peek() {
            if let Some(namespace) = self.get_namespace_by_id(&current_namespace_id) {
                // Get the child namespace ID by name
                if let Some(child_id) = namespace.get_id_by_name(&name.symbol) {
                    path_iter.next();
                    current_namespace_id = child_id;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        (current_namespace_id, path_iter.collect())
    }
}
