//
// Copyright 2025 Shuntaro Kasatani
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

use std::{fmt::Display, ops::Index};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SymbolPath {
    pub components: Vec<SymbolPathComponent>,
}

impl SymbolPath {
    pub fn new() -> Self {
        SymbolPath {
            components: Vec::new(),
        }
    }

    pub fn push(&mut self, component: SymbolPathComponent) {
        self.components.push(component);
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
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(".")
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum SymbolPathComponent {
    CompInt,
    CompFloat,
    CompBool,
    Var(String),
    StateVar(String),
    InputVar(String),
    OutputVar(String),
    Func(String),
    TypeDef(String),
    FuncParam(String),
}

impl Display for SymbolPathComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolPathComponent::CompInt => write!(f, "CompInt"),
            SymbolPathComponent::CompFloat => write!(f, "CompFloat"),
            SymbolPathComponent::CompBool => write!(f, "CompBool"),
            SymbolPathComponent::Var(name)
            | SymbolPathComponent::StateVar(name)
            | SymbolPathComponent::InputVar(name)
            | SymbolPathComponent::OutputVar(name)
            | SymbolPathComponent::Func(name)
            | SymbolPathComponent::TypeDef(name)
            | SymbolPathComponent::FuncParam(name) => write!(f, "{}", name),
        }
    }
}

// Use this macro to create a SymbolPath from a simple list of components
// Example:
// ```
// symbol_path!["foo", "bar", "baz"];
// ```
#[macro_export]
macro_rules! symbol_path {
    ( $( $x:expr ),* $(,)? ) => {
        {
            let mut temp_path = $crate::ast::SymbolPath::new();
            $(
                let temp_val = $x;
                // Type check to ensure it's SymbolPathComponent
                let _: &$crate::ast::SymbolPathComponent = &temp_val;
                // Push the component to the vector
                temp_path.push(temp_val);
            )*
            temp_path
        }
    };
}
