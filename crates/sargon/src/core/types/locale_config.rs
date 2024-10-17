use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocaleConfig {
    pub decimal_separator: Option<String>,
    pub grouping_separator: Option<String>,
}

impl LocaleConfig {
    pub fn new(
        decimal_separator: impl Into<Option<String>>,
        grouping_separator: impl Into<Option<String>>,
    ) -> Self {
        Self {
            decimal_separator: decimal_separator.into(),
            grouping_separator: grouping_separator.into(),
        }
    }

    pub fn with(
        decimal_separator: impl AsRef<str>,
        grouping_separator: impl AsRef<str>,
    ) -> Self {
        Self::new(
            Some(decimal_separator.as_ref().to_owned()),
            Some(grouping_separator.as_ref().to_owned()),
        )
    }
}

impl Default for LocaleConfig {
    fn default() -> Self {
        Self::with(".", " ")
    }
}

impl LocaleConfig {
    pub fn swedish_sweden() -> Self {
        Self::with(",", "\u{a0}")
    }
    pub fn english_united_states() -> Self {
        Self::with(".", ",")
    }
}

impl HasSampleValues for LocaleConfig {
    fn sample() -> Self {
        Self::swedish_sweden()
    }

    fn sample_other() -> Self {
        Self::english_united_states()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn swedish() {
        assert_eq!(
            LocaleConfig::swedish_sweden().decimal_separator.unwrap(),
            ","
        );
        assert_eq!(
            LocaleConfig::swedish_sweden().grouping_separator.unwrap(),
            "\u{a0}"
        );
    }

    #[test]
    fn english_us() {
        assert_eq!(
            LocaleConfig::english_united_states()
                .decimal_separator
                .unwrap(),
            "."
        );
        assert_eq!(
            LocaleConfig::english_united_states()
                .grouping_separator
                .unwrap(),
            ","
        );
    }

    #[test]
    fn default_uses_spaces_as_grouping_separator() {
        let sut = LocaleConfig::default();
        assert_eq!(&sut.grouping_separator.unwrap(), " ");
    }

    #[test]
    fn default_uses_dot_as_decimal_separator() {
        let sut = LocaleConfig::default();
        assert_eq!(&sut.decimal_separator.unwrap(), ".");
    }
}
