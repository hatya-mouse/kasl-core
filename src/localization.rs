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

//! Privodes the basic error localization functionality.

use crate::error::{EK, ErrorRecord, Pl};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "locales/"]
struct Locales;

/// Formats an error message based on the provided `ErrorRecord` and locale.
///
/// # Usage
/// ```rust
/// use kasl_core::{
///     ast_nodes::Range,
///     error::{EK, ErrorKey, ErrorRecord, Ph, Pl, Sv},
///     localization::format_error,
/// };
/// use std::collections::HashSet;
///
/// // Create a dummy error record for demonstration
/// let record = ErrorRecord {
///     key: ErrorKey::new(
///         EK::DuplicateInfixFunc,
///         Pl::StrVec(vec!["+".to_string(), "Int".to_string(), "Float".to_string()]),
///     ),
///     earliest_phase: Ph::GlobalDeclCollection,
///     ranges: [Range::n(0, 10)].iter().cloned().collect::<HashSet<_>>(),
///     severity: Sv::Error,
/// };
///
/// let message = format_error(&record, "en");
/// println!("{}", message);
/// ```
pub fn format_error(record: &ErrorRecord, locale: &str) -> String {
    // Create a file name by combining the locale and .toml extension
    let filename = format!("{}.toml", locale);
    // Get the file, falling back to English if not found
    let file = Locales::get(&filename).unwrap_or_else(|| Locales::get("en.toml").unwrap());

    // Get the localization table
    let content = str::from_utf8(file.data.as_ref()).unwrap();
    let table: toml::Table = toml::from_str(content).unwrap();

    if EK::ParserError == record.key.kind {
        format_parse_error(record, &table)
    } else {
        // Look up the template string
        let template = table["errors"][&record.key.kind.to_string()]
            .as_str()
            .unwrap_or("Unknown error");
        apply_payload(template, &record.key.payload.to_vec())
    }
}

fn apply_payload(template: &str, args: &[String]) -> String {
    let mut result = template.to_string();
    for (i, arg) in args.iter().enumerate() {
        result = result.replace(&format!("{{${}}}", i), arg);
    }
    result
}

fn format_parse_error(record: &ErrorRecord, table: &toml::Table) -> String {
    // Filter out tokens that are not meaningful for the user
    let Pl::StrVec(tokens) = record.key.payload.clone() else {
        return "".to_string();
    };
    let filtered_tokens: Vec<String> = tokens
        .into_iter()
        .filter(|token| {
            !matches!(
                token.as_str(),
                "\"\\n\"" | "\"#\"" | "EOF" | "STATEMENT" | "EXPRESSION"
            )
        })
        .collect();
    let Some((last, elements)) = filtered_tokens.split_last() else {
        return "".to_string();
    };
    let first = elements.join(", ");

    // Get the expected template based on the number of filtered tokens
    let expected_template = if filtered_tokens.len() == 1 {
        table["parser_error"]["SingleExpected"]
            .as_str()
            .unwrap_or("Unknown error")
    } else {
        table["parser_error"]["MultipleExpected"]
            .as_str()
            .unwrap_or("Unknown error")
    };
    apply_payload(expected_template, &[first, last.to_string()])
}
