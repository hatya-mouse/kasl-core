//
// © 2025 Shuntaro Kasatani
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

use std::fmt::{Debug, Formatter};

use crate::{Function, Program, TypeDef};

pub enum ScopeItem<'a> {
    Program(&'a Program),
    TypeDef(&'a TypeDef),
    Func(&'a Function),
}

impl Debug for ScopeItem<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeItem::Program(_) => write!(f, "Program"),
            ScopeItem::TypeDef(td) => write!(f, "TypeDef({:?})", td.name),
            ScopeItem::Func(func) => write!(f, "Func({:?})", func.name),
        }
    }
}

pub enum ScopeItemMut<'a> {
    Program(&'a mut Program),
    TypeDef(&'a mut TypeDef),
    Func(&'a mut Function),
}

impl Debug for ScopeItemMut<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeItemMut::Program(_) => write!(f, "Program"),
            ScopeItemMut::TypeDef(td) => write!(f, "TypeDef({:?})", td.name),
            ScopeItemMut::Func(func) => write!(f, "Func({:?})", func.name),
        }
    }
}
