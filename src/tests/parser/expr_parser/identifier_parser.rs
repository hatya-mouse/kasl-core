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

use crate::tests::common::parser::parse_expr;
use insta::assert_debug_snapshot;

#[test]
fn parse_single_identifier() {
    let code = "let value = num";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_identifier_access() {
    let code = "let value = type.num";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_identifier_access_to_func() {
    let code = "let value = new().first";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_func_access() {
    let code = "let value = new()";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_func_access_to_identifier() {
    let code = "let value = num.new()";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_func_chain() {
    let code = "let value = func().chain().new()";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_func_chain_on_expr() {
    let code = "let value = (1 + 7).func().chain().new()";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}
