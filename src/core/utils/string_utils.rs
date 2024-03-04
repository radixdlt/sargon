/// Returns the last `n` chars of the &str `s`. If `s` is shorter than `n`
/// we panic.
pub fn suffix_str(n: usize, s: impl AsRef<str>) -> String {
    let split_pos = s.as_ref().char_indices().nth_back(n - 1).unwrap().0;
    s.as_ref()[split_pos..].to_string()
}

/// Capitalizes the first character in s.
pub fn capitalize(s: impl AsRef<str>) -> String {
    let mut c = s.as_ref().chars();
    match c.next() {
        None => String::new(),
        Some(f) => format!("{}{}", f.to_uppercase(), c.as_str()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_suffix() {
        assert_eq!(suffix_str(8, "Radix... imagine!"), "imagine!")
    }

    #[test]
    fn string_suffix() {
        assert_eq!(suffix_str(7, "By the rivers of Babylon"), "Babylon")
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("g"), "G");
        assert_eq!(capitalize("good"), "Good");
    }
}
