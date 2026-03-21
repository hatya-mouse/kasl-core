use std::{fmt::Display, path::PathBuf};

#[derive(Debug, PartialEq, Clone)]
pub struct ImportPath {
    pub path: Vec<String>,
}

impl Display for ImportPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.join("/"))
    }
}

impl ImportPath {
    pub fn to_path(&self) -> PathBuf {
        PathBuf::from(self.path.join("/"))
    }
}
