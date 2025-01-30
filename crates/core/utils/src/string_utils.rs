use url::form_urlencoded;

/// Returns the last `n` chars of the &str `s`. If `s` is shorter than `n`
/// we panic.
pub fn suffix_str(n: usize, s: impl AsRef<str>) -> String {
    let split_pos = s.as_ref().char_indices().nth_back(n - 1).unwrap().0;
    s.as_ref()[split_pos..].to_string()
}

fn _type_name<T>() -> String {
    std::any::type_name::<T>()
        .split("::")
        .last()
        .unwrap()
        .to_owned()
}
pub fn type_name<T>() -> String {
    _type_name::<T>()
}

pub trait TypeName {
    fn type_name() -> String;
}

impl<T> TypeName for T {
    fn type_name() -> String {
        _type_name::<T>()
    }
}

pub fn format_string(s: impl AsRef<str>, start: usize, end: usize) -> String {
    let s = s.as_ref();
    let prefix = &s[0..start];
    let suffix = suffix_str(end, s);
    format!("{}...{}", prefix, suffix)
}

/// Returns the first `n` chars of the &str `s`. If `n` is bigger than `s` then
/// the whole `s` is returned.
pub fn prefix_str(n: usize, s: impl AsRef<str>) -> String {
    s.as_ref().chars().take(n).collect()
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

pub fn url_encode(s: impl AsRef<str>) -> String {
    form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
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
        assert_eq!(
            suffix_str(7, String::from("By the rivers of Babylon")),
            "Babylon"
        )
    }

    #[test]
    fn str_prefix() {
        assert_eq!(prefix_str(8, "Radix... imagine!"), "Radix...")
    }

    #[test]
    fn str_prefix_longer_n() {
        assert_eq!(prefix_str(32, "Radix... imagine!"), "Radix... imagine!")
    }

    #[test]
    fn string_prefix() {
        assert_eq!(
            prefix_str(7, String::from("By the rivers of Babylon")),
            "By the "
        )
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
    fn test_url_encode() {
        let url = "https://svgshare.com/i/U7z.svg";
        let data_url = "data:image/svg+xml,%3Csvg%20viewBox%3D%220%200%201000%201000%22%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%3E%0A%3Cpolygon%20fill%3D%22hsla%2890%2C99%25%2C52%25%2C1%29%22%20points%3D%220%2C%200%2C%201000%2C%201000%2C%200%2C%201000%22%20transform%3D%22scale%28-1%2C1%29%20translate%28-1000%29%22%2F%3E%0A%3Cpolygon%20fill%3D%22hsla%28199%2C90%25%2C64%25%2C1%29%22%20points%3D%221000%2C%201000%2C%201000%2C%200%2C%200%2C%200%22%20transform%3D%22scale%28-1%2C1%29%20translate%28-1000%29%22%2F%3E%0A%3Cpath%20d%3D%22M1000%2C229%20A1000%2C1000%2C0%2C0%2C0%2C229%2C1000%20L1000%2C1000%20z%22%20fill%3D%22hsla%28140%2C98%25%2C61%25%2C1%29%22%2F%3E%0A%3Cpath%20d%3D%22M392%2C500%20L608%2C500%20M500%2C392%20L500%2C608%22%20stroke%3D%22hsla%2847%2C92%25%2C61%25%2C1%29%22%20stroke-width%3D%2272%22%2F%3E%0A%3C%2Fsvg%3E";

        pretty_assertions::assert_eq!(
            url_encode(url),
            "https%3A%2F%2Fsvgshare.com%2Fi%2FU7z.svg"
        );
        pretty_assertions::assert_eq!(
            url_encode(data_url),
            "data%3Aimage%2Fsvg%2Bxml%2C%253Csvg%2520viewBox%253D%25220%25200%25201000%25201000%2522%2520xmlns%253D%2522http%253A%252F%252Fwww.w3.org%252F2000%252Fsvg%2522%253E%250A%253Cpolygon%2520fill%253D%2522hsla%252890%252C99%2525%252C52%2525%252C1%2529%2522%2520points%253D%25220%252C%25200%252C%25201000%252C%25201000%252C%25200%252C%25201000%2522%2520transform%253D%2522scale%2528-1%252C1%2529%2520translate%2528-1000%2529%2522%252F%253E%250A%253Cpolygon%2520fill%253D%2522hsla%2528199%252C90%2525%252C64%2525%252C1%2529%2522%2520points%253D%25221000%252C%25201000%252C%25201000%252C%25200%252C%25200%252C%25200%2522%2520transform%253D%2522scale%2528-1%252C1%2529%2520translate%2528-1000%2529%2522%252F%253E%250A%253Cpath%2520d%253D%2522M1000%252C229%2520A1000%252C1000%252C0%252C0%252C0%252C229%252C1000%2520L1000%252C1000%2520z%2522%2520fill%253D%2522hsla%2528140%252C98%2525%252C61%2525%252C1%2529%2522%252F%253E%250A%253Cpath%2520d%253D%2522M392%252C500%2520L608%252C500%2520M500%252C392%2520L500%252C608%2522%2520stroke%253D%2522hsla%252847%252C92%2525%252C61%2525%252C1%2529%2522%2520stroke-width%253D%252272%2522%252F%253E%250A%253C%252Fsvg%253E"
        );
    }

    #[test]
    fn test_typename() {
        struct GreatStruct {}
        assert_eq!(type_name::<GreatStruct>(), "GreatStruct");
    }
}
