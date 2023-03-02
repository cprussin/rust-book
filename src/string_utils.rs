pub fn str_when(condition: bool, string: &str) -> &str {
    if condition {
        string
    } else {
        ""
    }
}
