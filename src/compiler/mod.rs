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

use cranelift_codegen::ir::{self, types};
pub const TYPE_INT: ir::Type = types::I64;
pub const TYPE_FLOAT: ir::Type = types::F32;

pub mod codegen;
pub mod compiler;
pub mod functions;
pub mod run;

pub use codegen::Translator;
pub use compiler::Compiler;
pub use run::Executable;
