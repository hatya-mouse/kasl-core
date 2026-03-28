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

pub mod blueprint_builder;
pub mod error;
pub mod expr_engine;
pub mod global_decl_collection;
pub mod scope_graph_analyzing;
pub mod statement_building;
pub mod struct_graph_analyzing;

mod common_utils;
mod namespace_constructor;

pub use blueprint_builder::BlueprintBuilder;
pub use global_decl_collection::GlobalDeclCollector;
pub use scope_graph_analyzing::ScopeGraphAnalyzer;
pub use statement_building::StatementBuilder;
pub use struct_graph_analyzing::StructGraphAnalyzer;
