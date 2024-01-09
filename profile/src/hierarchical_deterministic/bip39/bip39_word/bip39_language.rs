/// Language to be used for the mnemonic phrase.
///
/// The English language is always available, other languages are enabled using
/// the compilation features.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, uniffi::Enum)]
pub enum BIP39Language {
    /// The English language.
    English,
}

impl Default for BIP39Language {
    fn default() -> Self {
        Self::English
    }
}

impl From<bip39::Language> for BIP39Language {
    fn from(value: bip39::Language) -> Self {
        use bip39::Language::*;
        match value {
            English => Self::English,
        }
    }
}
impl From<BIP39Language> for bip39::Language {
    fn from(value: BIP39Language) -> Self {
        use bip39::Language::*;
        match value {
            BIP39Language::English => English,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BIP39Language;

    #[test]
    fn default_is_english() {
        assert_eq!(BIP39Language::default(), BIP39Language::English);
    }

    #[test]
    fn into() {
        assert_eq!(BIP39Language::English, bip39::Language::English.into());
        assert_eq!(
            bip39::Language::from(BIP39Language::English),
            bip39::Language::English
        );
    }
}
