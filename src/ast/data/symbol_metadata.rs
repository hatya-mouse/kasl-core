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

use crate::data::SymbolID;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolMetadata {
    pub symbol_type: Option<SymbolID>,
}

impl SymbolMetadata {
    pub fn new(symbol_type: Option<SymbolID>) -> Self {
        Self { symbol_type }
    }

    pub fn with_type(symbol_type: SymbolID) -> Self {
        Self {
            symbol_type: Some(symbol_type),
        }
    }

    pub fn no_type() -> Self {
        Self { symbol_type: None }
    }
}
