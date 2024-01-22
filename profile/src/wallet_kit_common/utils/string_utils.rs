/// Returns the last `n` chars of the &str `s`. If `s` is shorter than `n`
/// we panic.
pub fn suffix_str(n: usize, s: impl AsRef<str>) -> String {
    let split_pos = s.as_ref().char_indices().nth_back(n - 1).unwrap().0;
    s.as_ref()[split_pos..].to_string()
}

#[cfg(test)]
mod tests {
    use crate::suffix_str;

    #[test]
    fn str_suffix() {
        assert_eq!(suffix_str(8, "Radix... imagine!"), "imagine!")
    }

    #[test]
    fn string_suffix() {
        assert_eq!(suffix_str(7, "By the rivers of Babylon"), "Babylon")
    }
}
