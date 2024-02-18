use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
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
}

impl Default for LocaleConfig {
    fn default() -> Self {
        Self::new(".".to_owned(), " ".to_owned())
    }
}

impl From<num_format::Locale> for LocaleConfig {
    fn from(value: num_format::Locale) -> Self {
        Self::new(
            Some(value.decimal().to_owned()),
            Some(value.separator().to_owned()),
        )
    }
}

impl LocaleConfig {
    /// Tries to create a
    /// A BCP-47 language identifier such as `"en_US_POSIX"`, `"sv_FI"` or `"zh_Hant_MO"`,
    /// see: [list][list]
    ///
    /// [list]: https://docs.rs/num-format/0.4.4/src/num_format/locale.rs.html#5565-6444
    pub fn from_identifier(identifier: impl AsRef<str>) -> Result<Self> {
        let identifier = identifier.as_ref().to_owned();
        num_format::Locale::from_name(identifier.clone())
            .map_err(|_| CommonError::UnrecognizedLocaleIdentifier {
                bad_value: identifier,
            })
            .map(Into::<Self>::into)
    }
}

#[cfg(test)]
impl LocaleConfig {
    pub fn swedish() -> Self {
        Self::from_identifier("sv").expect("Sweden exists")
    }
    pub fn us() -> Self {
        Self::from_identifier("en_US_POSIX").expect("US exists")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn swedish() {
        assert_eq!(LocaleConfig::swedish().decimal_separator.unwrap(), ",");
        assert_eq!(
            LocaleConfig::swedish().grouping_separator.unwrap(),
            "\u{a0}"
        );
    }

    #[test]
    fn english_us() {
        assert_eq!(LocaleConfig::us().decimal_separator.unwrap(), ".");
        assert_eq!(LocaleConfig::us().grouping_separator.unwrap(), ",");
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

    #[test]
    fn from_identifier_invalid() {
        assert_eq!(
            LocaleConfig::from_identifier("foo"),
            Err(CommonError::UnrecognizedLocaleIdentifier {
                bad_value: "foo".to_owned()
            })
        );
    }
}
