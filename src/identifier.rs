pub fn is_alpha_numeric(c: char) -> bool {
    c.is_ascii_digit() || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}
