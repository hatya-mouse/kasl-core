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

use crate::Expression;
use knodiq_engine::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Time,
    Input,
    Output,
    Variable,
    Function,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub kind: SymbolKind,
    pub initial_value: Option<Expression>,
    pub range: Option<(f32, f32)>,
    pub value: Option<Value>,
}
