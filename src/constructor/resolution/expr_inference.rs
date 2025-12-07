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

use crate::{ConstructorError, ConstructorErrorType, ExprToken, Program, Range, SymbolPath};

pub fn infer_expr_type(
    program: &Program,
    expr: &[ExprToken],
) -> Result<SymbolPath, ConstructorError> {
    // todo!("Implement Expression Type Inference");
    Err(ConstructorError {
        error_type: ConstructorErrorType::Placeholder,
        position: Range::zero(),
    })
}
