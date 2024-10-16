pub fn escape_json(value: &str) -> String {
    replace_all(
        value,
        &["\\", "\"", r"\b", r"\f", "\n", "\r", "\t"],
        &["\\\\", "\\\"", "\\b", "\\f", "\\n", "\\r", "\\t"],
    )
}

#[inline]
fn replace_all(input: &str, from: &[&str], to: &[&str]) -> String {
    let mut result = input.to_string();
    for (pattern, replacement) in from.iter().zip(to) {
        result = result.replace(pattern, replacement);
    }
    result
}
