//
// © 2025-2026 Shuntaro Kasatani
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

use crate::NameSpaceID;

/// An set of NameSpaceID and other symbol ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, serde::Serialize)]
pub struct NameSpacePair<T> {
    pub namespace_id: NameSpaceID,
    pub symbol_id: T,
}

impl<T> NameSpacePair<T> {
    pub fn new(namespace_id: NameSpaceID, symbol_id: T) -> Self {
        Self {
            namespace_id,
            symbol_id,
        }
    }
}
