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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    /// Title: DuplicateSymbol
    /// Phase: SymbolTableConstruction
    /// Payload: Name of the duplicate symbol
    DuplicateSymbol,

    /// Title: RequiredByOutsideType
    /// Phase: TopLevelCollection
    /// Payload: None
    RequiredByOutsideType,

    /// Title: InvalidParamNumbersForInfix
    /// Phase: TopLevelCollection
    /// Payload: None
    InvalidParamNumbersForInfix,

    /// Title: InvalidParamNumbersForPrefix
    /// Phase: TopLevelCollection
    /// Payload: None
    InvalidParamNumbersForPrefix,

    /// Title: DuplicateLiteralBind
    /// Phase: MemberCollection
    /// Payload: Type of the duplicate literal bind
    DuplicateLiteralBind,

    /// Title: CompilerBug
    /// Payload: Error message
    CompilerBug,
}
