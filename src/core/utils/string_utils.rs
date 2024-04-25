use crate::CommonError;
use url::Url;

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

pub trait StrExt {
    fn remove_last(&self) -> &str;
}

impl StrExt for str {
    fn remove_last(&self) -> &str {
        match self.char_indices().next_back() {
            Some((i, _)) => &self[..i],
            None => self,
        }
    }
}

pub fn parse_url(s: impl AsRef<str>) -> Result<Url, CommonError> {
    Url::try_from(s.as_ref()).map_err(|_| CommonError::InvalidURL {
        bad_value: s.as_ref().to_owned(),
    })
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

    #[test]
    fn remove_last_char() {
        assert_eq!("".remove_last(), "");
        assert_eq!("x".remove_last(), "");
        assert_eq!("X".remove_last(), "");
        assert_eq!("1".remove_last(), "");
        assert_eq!("a".remove_last(), "");
        assert_eq!("fo".remove_last(), "f");
        assert_eq!("Foobar".remove_last(), "Fooba");
    }

    #[test]
    fn test_parse_url() {
        assert!(parse_url("https://radixdlt.com").is_ok());
    }

    #[test]
    fn test_parse_url_invalid() {
        assert!(parse_url("https/radixdlt").is_err());
    }
}
