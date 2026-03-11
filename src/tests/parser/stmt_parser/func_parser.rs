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
fn parse_simple_func() {
    let code = "func main() {}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_func_with_param() {
    let code = "func main(label name: Type) {}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_func_with_return() {
    let code = "func main() -> Return {}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_func_with_param_and_return() {
    let code = "func main(label name: Type) -> Return {}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_func_with_many_params() {
    let code = "func main(label1 name1: Type, label2 name2: Type, label3 name3: Type) -> Return {}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}
