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

use crate::common::parser::parse_expr;
use insta::assert_debug_snapshot;

#[test]
fn parse_simple_struct() {
    let code = "struct Type {}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_struct_with_field() {
    let code = "struct Type {
    var field: Int = 0
}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_struct_with_func() {
    let code = "struct Type {
    func process(label name: Type) -> Return {}
}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_struct_field_and_func() {
    let code = "struct Type {
    var field: Int = 0
    func process(label name: Type) -> Return {}
}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}
