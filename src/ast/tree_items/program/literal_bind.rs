//
// Copyright 2025-2026 Shuntaro Kasatani
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

use crate::{ConstructorError, ConstructorErrorType, LiteralBind, Program, Range, SymbolPath};

impl Program {
    /// Set a int literal type. If it already exists, this will return a ConstructorError.
    pub fn set_int_literal(&mut self, literal_type: SymbolPath) -> Result<(), ConstructorError> {
        if self.int_literal_type.is_some() {
            Err(ConstructorError {
                error_type: ConstructorErrorType::DuplicateLiteralBind(LiteralBind::IntLiteral),
                position: Range::zero(),
            })
        } else {
            self.int_literal_type = Some(literal_type);
            Ok(())
        }
    }

    /// Set a int literal initializer. If it already exists, this will return a ConstructorError.
    pub fn set_float_literal(&mut self, literal_type: SymbolPath) -> Result<(), ConstructorError> {
        if self.float_literal_type.is_some() {
            Err(ConstructorError {
                error_type: ConstructorErrorType::DuplicateLiteralBind(LiteralBind::FloatLiteral),
                position: Range::zero(),
            })
        } else {
            self.float_literal_type = Some(literal_type);
            Ok(())
        }
    }

    /// Set a int literal initializer. If it already exists, this will return a ConstructorError.
    pub fn set_bool_literal(&mut self, literal_type: SymbolPath) -> Result<(), ConstructorError> {
        if self.bool_literal_type.is_some() {
            Err(ConstructorError {
                error_type: ConstructorErrorType::DuplicateLiteralBind(LiteralBind::BoolLiteral),
                position: Range::zero(),
            })
        } else {
            self.bool_literal_type = Some(literal_type);
            Ok(())
        }
    }
}
