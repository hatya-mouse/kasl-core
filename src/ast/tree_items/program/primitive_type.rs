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

use crate::{PrimitiveType, Program, data::SymbolID, symbol_path};

impl Program {
    pub fn add_primitive_type(&mut self, ty: PrimitiveType) {
        let id = self.get_next_id();
        self.register_id(symbol_path![ty.to_string()], id);
        self.primitive_types.insert(id, ty);
    }

    pub fn get_id_of_primitive_type(&self, ty: &PrimitiveType) -> Option<SymbolID> {
        self.get_id_by_path(&symbol_path![ty.to_string()])
            .and_then(|ids| ids.first().copied())
    }
}
