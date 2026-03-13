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

use crate::common::{
    TestContext, analyze_structs, assert_error,
    builders::{struct_decl, struct_field},
    collect_global_decls,
};
use kasl::{error::EK, symbol_path};

// --- ERROR CASES ---

#[test]
fn test_self_cycle() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![struct_decl(
        "Type",
        &[struct_field(
            "field",
            Some(symbol_path!["Type".to_string()]),
            &[],
        )],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = analyze_structs(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::StructCycle);
}
