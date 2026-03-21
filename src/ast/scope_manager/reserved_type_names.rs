pub(super) fn is_reserved_name(name: &str) -> bool {
    matches!(name, "Int" | "Float" | "Bool" | "Builtin")
}
