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
    TestContext, analyze_scopes, assert_error, build_stmts,
    builders::{expression, func_call, func_decl},
    collect_global_decls,
};
use kasl::error::EK;

// --- ERROR CASES ---

#[test]
fn test_simple_recursion() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "do_something",
        &[],
        None,
        &[expression(&[func_call("do_something", &[])])],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    println!("{:#?}", &test_ctx.scope_graph);
    let error = analyze_scopes(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::RecursiveCall);
}
