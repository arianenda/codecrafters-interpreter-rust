pub fn is_digit(c: char) -> bool {
    c.is_ascii_digit() || c == '.'
}

pub fn format_decimal(s: &str) -> String {
    if !s.contains('.') {
        return format!("{}.0", s);
    }

    let trimmed = s.trim_end_matches('0');
    if trimmed.ends_with('.') {
        format!("{}0", trimmed)
    } else {
        trimmed.to_string()
    }
}
