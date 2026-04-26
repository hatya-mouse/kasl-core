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

//! The implementation of the AST construction phase of the compiler.

pub mod blueprint_builder;
pub mod decl_collection;
pub mod error;
pub mod expr_engine;
pub mod flow_analysis;
pub mod scope_analysis;
pub mod stmt_builder;

mod namespace_constructor;
mod utils;

pub use blueprint_builder::BlueprintBuilder;
pub use decl_collection::GlobalDeclCollector;
pub use scope_analysis::ScopeGraphAnalyzer;
pub use stmt_builder::StatementBuilder;
