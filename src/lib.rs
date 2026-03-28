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

pub(crate) const MAIN_FUNCTION_NAME: &str = "main";
pub(crate) const LOOP_UNROLL_THRESHOLD: u32 = 32;

pub mod ast;
pub mod ast_construction;
pub mod backend;
pub mod builtin;
pub mod compiler;
pub mod localization;
pub mod parser;
pub mod run_program;

pub use ast_construction::error;
pub use compiler::KaslCompiler;
pub use run_program::{run_buffer, run_once};
