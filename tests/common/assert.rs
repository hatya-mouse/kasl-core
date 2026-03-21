use kasl::error::{ErrorKind, ErrorRecord};

pub fn assert_error(error: &[ErrorRecord], expected: ErrorKind) {
    assert!(error.iter().any(|r| r.key.kind == expected))
}

#[macro_export]
macro_rules! assert_func_ctx_snapshot {
    ($func_ctx:expr) => {
        use insta::{assert_yaml_snapshot, sorted_redaction};
        assert_yaml_snapshot!($func_ctx, {
            ".funcs" => sorted_redaction(),
            ".member_functions" => sorted_redaction(),
            ".global_functions" => sorted_redaction()
        });
    };
}

#[macro_export]
macro_rules! assert_scope_registry_snapshot {
    ($scope_registry:expr) => {
        use insta::{assert_yaml_snapshot, sorted_redaction};
        assert_yaml_snapshot!($scope_registry, {
            ".scopes" => sorted_redaction(),
            ".variables" => sorted_redaction(),
            ".global_scope_ids" => sorted_redaction(),
            ".**.name_to_id" => sorted_redaction()
        });
    };
}

#[macro_export]
macro_rules! assert_type_registry_snapshot {
    ($type_registry:expr) => {
        use insta::{assert_yaml_snapshot, sorted_redaction};
        assert_yaml_snapshot!($type_registry, {
            ".structs" => sorted_redaction(),
            ".name_to_id" => sorted_redaction(),
            ".**.indices" => sorted_redaction(),
        });
    };
}
