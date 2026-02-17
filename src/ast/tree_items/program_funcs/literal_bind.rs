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
    LiteralBind, Program, Range, SymbolPath,
    error::{ErrorCollector, Phase},
};

impl Program {
    /// Set a int literal type. If it already exists, this will throw an error.
    pub fn set_int_literal(
        &mut self,
        ec: &mut ErrorCollector,
        literal_type: SymbolPath,
        func_range: Range,
    ) {
        if self.int_literal_type.is_some() {
            ec.dup_literal_bind(func_range, Phase::TypeResolution, LiteralBind::IntLiteral);
        } else {
            self.int_literal_type = Some(literal_type);
        }
    }

    /// Set a int literal initializer. If it already exists, this will throw an error.
    pub fn set_float_literal(
        &mut self,
        ec: &mut ErrorCollector,
        literal_type: SymbolPath,
        func_range: Range,
    ) {
        if self.int_literal_type.is_some() {
            ec.dup_literal_bind(func_range, Phase::TypeResolution, LiteralBind::FloatLiteral);
        } else {
            self.float_literal_type = Some(literal_type);
        }
    }

    /// Set a int literal initializer. If it already exists, this will throw an error.
    pub fn set_bool_literal(
        &mut self,
        ec: &mut ErrorCollector,
        literal_type: SymbolPath,
        func_range: Range,
    ) {
        if self.int_literal_type.is_some() {
            ec.dup_literal_bind(func_range, Phase::TypeResolution, LiteralBind::BoolLiteral);
        } else {
            self.bool_literal_type = Some(literal_type);
        }
    }
}
