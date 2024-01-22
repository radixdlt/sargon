/// Returns the last `n` chars of the &str `s`. If `s` is shorter than `n`
/// we panic.
pub fn suffix_str(n: usize, s: &str) -> String {
    let split_pos = s.char_indices().nth_back(n - 1).unwrap().0;
    s[split_pos..].to_string()
}

/// Returns the last `n` chars of the &String `s`. If `s` is shorter than `n`
/// we panic.
pub fn suffix_string(n: usize, s: &String) -> String {
    suffix_str(n, s)
}

#[cfg(test)]
mod tests {
    use crate::{suffix_str, suffix_string};

    #[test]
    fn str_suffix() {
        assert_eq!(suffix_str(8, "Radix... imagine!"), "imagine!")
    }

    #[test]
    fn string_suffix() {
        assert_eq!(
            suffix_string(7, &"By the rivers of Babylon".to_string()),
            "Babylon"
        )
    }
}
