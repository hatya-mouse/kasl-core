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

use crate::{
    Program, Range, SymbolPath,
    data::SymbolID,
    error::{ErrorCollector, Ph},
};

pub trait SymbolTypeGetter {
    fn get_first_type_from_path<F>(
        &self,
        ec: &mut ErrorCollector,
        symbol_path: &SymbolPath,
        range: Range,
        error_fn: F,
    ) -> Option<SymbolID>
    where
        F: Fn(&mut ErrorCollector, &SymbolPath);
}

impl SymbolTypeGetter for Program {
    fn get_first_type_from_path<F>(
        &self,
        ec: &mut ErrorCollector,
        symbol_path: &SymbolPath,
        range: Range,
        error_fn: F,
    ) -> Option<SymbolID>
    where
        F: Fn(&mut ErrorCollector, &SymbolPath),
    {
        let symbol_id = match self.get_id_by_path(symbol_path).and_then(|ids| ids.first()) {
            Some(id) => id,
            None => {
                error_fn(ec, symbol_path);
                return None;
            }
        };
        let symbol_type = match self.get_symbol_type(symbol_id) {
            Some(symbol_type) => symbol_type,
            None => {
                ec.comp_bug(
                    range,
                    Ph::TypeResolution,
                    &format!(
                        "No symbol type found for ID \"{}\" which corresponds to path \"{}\"",
                        symbol_id, symbol_path
                    ),
                );
                return None;
            }
        };
        Some(symbol_type)
    }
}
