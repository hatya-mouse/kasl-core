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
fn parse_input_no_type() {
    let code = "input in = 0";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_input_with_type() {
    let code = "input in: Int = 0";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_output_no_type() {
    let code = "output out = 0";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_output_with_type() {
    let code = "output out: Int = 0";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_state_no_type() {
    let code = "state delay = 0";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_state_with_type() {
    let code = "state delay: Int = 0";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_const_no_type() {
    let code = "let factor = 0";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_const_with_type() {
    let code = "let factor: Int = 0";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}
