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

/// Payload for error messages.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Payload {
    None,
    Num(usize),
    StrAndNum(String, usize),
    StrVec(Vec<String>),
}

impl Payload {
    pub fn to_vec(&self) -> Vec<String> {
        match self {
            Payload::None => vec![],
            Payload::Num(a) => vec![a.to_string()],
            Payload::StrAndNum(a, b) => vec![a.clone(), b.to_string()],
            Payload::StrVec(a) => a.clone(),
        }
    }
}
