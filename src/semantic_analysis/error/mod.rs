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

//! Collects and organizes the emitted errors during the AST construction phase.

pub mod error_collector;
pub mod error_kind;
pub mod error_record;
pub mod payload;
mod wrappers;

pub use error_collector::ErrorCollector;
pub use error_kind::ErrorKind;
pub use error_record::{ErrorKey, ErrorRecord, Phase, Severity};
pub use payload::Payload;

/// A shorthand for `ErrorKind`.
pub type EK = crate::error::ErrorKind;
/// A shorthand for `Phase`.
pub type Ph = crate::error::Phase;
/// A shorthand for `Severity`.
pub type Sv = crate::error::Severity;
/// A shorthand for `Payload`.
pub type Pl = crate::error::Payload;
