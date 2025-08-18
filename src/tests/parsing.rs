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

#[cfg(test)]
mod parsing {
    use crate::{ExprToken, kash};

    #[test]
    fn chaining() {
        assert_eq!(
            kash::expression("object"),
            Ok(vec![ExprToken::Identifier(vec!["object".to_string()])])
        );

        assert_eq!(
            kash::expression("object.property"),
            Ok(vec![ExprToken::Identifier(vec![
                "object".to_string(),
                "property".to_string()
            ])])
        );

        assert_eq!(
            kash::expression("object.property.subproperty"),
            Ok(vec![ExprToken::Identifier(vec![
                "object".to_string(),
                "property".to_string(),
                "subproperty".to_string()
            ])])
        );

        assert_eq!(
            kash::expression("object.method(param1, param2)"),
            Ok(vec![ExprToken::FuncCall {
                name: vec!["object".to_string(), "method".to_string()],
                args: vec![
                    vec![ExprToken::Identifier(vec!["param1".to_string()])],
                    vec![ExprToken::Identifier(vec!["param2".to_string()])]
                ]
            }])
        );
    }
}
