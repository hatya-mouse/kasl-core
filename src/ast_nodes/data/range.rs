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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, serde::Serialize)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Default for Range {
    fn default() -> Self {
        Self::zero()
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl Range {
    pub fn n(start: usize, end: usize) -> Self {
        Range { start, end }
    }

    pub fn zero() -> Self {
        Range { start: 0, end: 0 }
    }
}
