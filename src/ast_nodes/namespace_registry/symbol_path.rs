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

use std::{fmt::Display, ops::Index};

/// A absolute path to a symbol in the symbol table.
#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize)]
pub struct SymbolPath {
    components: Vec<SymbolPathComponent>,
}

impl Default for SymbolPath {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolPath {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn with(components: Vec<SymbolPathComponent>) -> Self {
        Self { components }
    }

    pub fn push(&mut self, component: SymbolPathComponent) {
        self.components.push(component);
    }

    pub fn extended(&self, component: Vec<SymbolPathComponent>) -> Self {
        let mut new_path = self.clone();
        new_path.components.extend(component);
        new_path
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }

    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    pub fn first(&self) -> Option<&SymbolPathComponent> {
        self.components.first()
    }

    pub fn last(&self) -> Option<&SymbolPathComponent> {
        self.components.last()
    }
}

impl Index<usize> for SymbolPath {
    type Output = SymbolPathComponent;

    fn index(&self, index: usize) -> &Self::Output {
        &self.components[index]
    }
}

impl Display for SymbolPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.components
                .iter()
                .map(|p| p.symbol.clone())
                .collect::<Vec<_>>()
                .join(".")
        )
    }
}

impl IntoIterator for SymbolPath {
    type Item = SymbolPathComponent;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.into_iter()
    }
}

impl FromIterator<SymbolPathComponent> for SymbolPath {
    fn from_iter<I: IntoIterator<Item = SymbolPathComponent>>(iter: I) -> Self {
        Self {
            components: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize)]
pub struct SymbolPathComponent {
    pub symbol: String,
}

impl SymbolPathComponent {
    pub fn new(symbol: String) -> Self {
        Self { symbol }
    }
}

/// Creates a SymbolPath from a list of strings.
///
/// # Usage
/// ```rust
/// use kasl_core::symbol_path;
///
/// let path = symbol_path!["foo".to_string(), "bar".to_string(), "baz".to_string()];
/// assert_eq!(path.to_string(), "foo.bar.baz");
/// ```
#[macro_export]
macro_rules! symbol_path {
    ($($x:expr),* $(,)?) => {
        {
            let mut temp_path = $crate::ast_nodes::SymbolPath::new();
            $(
                let temp_val = $x;
                // Type check to ensure it's a String
                let _: &String = &temp_val;
                // Push the component to the vector
                temp_path.push($crate::ast_nodes::SymbolPathComponent { symbol: temp_val.to_string() });
            )*
            temp_path
        }
    };
}
