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

//! This crate includes the core implementation of the KASL programming language, and also provides an easy-to-use API for compiling and running KASL programs.
//!
//! # Getting Started
//!
//! To run a KASL program from your Rust code, you can use the `KaslCompiler` struct to compile the program, and `run_once` or `run_buffer` functions to execute it.
//! Here's a simple example:
//!
//! ```rust
//! use kasl::{KaslCompiler, run_once};
//!
//! let code = r#"
//! operator infix + {
//!     precedence: 10,
//!     associativity: left
//! }
//!
//! func infix +(lhs: Int, rhs: Int) -> Int {
//!     return Builtin.iadd(lhs, rhs)
//! }
//!
//! input in_value = 0
//! output out_value = 0
//!
//! func main() {
//!     // Increment the in_value by one.
//!     out_value = in_value + 1
//! }
//! "#;
//!
//! // Create a new instance of the compiler
//! let mut compiler = KaslCompiler::default();
//!
//! // Parse the KASL code
//! compiler.parse(code).expect("Failed to parse code");
//!
//! // Analyze and build the program, which returns an IOBlueprint
//! let blueprint = compiler.build().expect("Failed to build program");
//!
//! // Create a input value pointer
//! let in_value = 5;
//! let in_value_ptr = &in_value as *const i32 as *const ();
//!
//! // Allocate a memory for the output value based on the blueprint
//! let out_value_size = blueprint.get_outputs()[0].actual_size;
//! let out_value_ptr = unsafe {
//!     let layout = std::alloc::Layout::from_size_align(out_value_size, 1).unwrap();
//!     std::alloc::alloc(layout) as *mut ()
//! };
//!
//! // Compile the program
//! let program_ptr = compiler.compile_once(&blueprint).expect("Failed to compile program");
//!
//! // Run the program
//! unsafe {
//!     run_once(program_ptr, &[in_value_ptr], &[out_value_ptr], &[], 1);
//! }
//!
//! assert_eq!(unsafe { *(out_value_ptr as *const i32) }, 6);
//! ```

pub(crate) const MAIN_FUNCTION_NAME: &'static str = "main";
pub(crate) const LOOP_UNROLL_THRESHOLD: u32 = 32;

pub mod ast;
pub mod ast_construction;
pub mod builtin;
pub mod compiler;
pub mod localization;
pub mod lowerer;
pub mod parser;
pub mod run_program;

pub use ast_construction::error;
pub use compiler::KaslCompiler;
pub use run_program::{run_buffer, run_once};
