use std::convert::Infallible;

use crate::prelude::*;

/// A BIP39 passphrase, which required but when not used by user, the Default value will be use (empty string),
/// as per BIP39 standard.
#[derive(
    Serialize, Deserialize, Clone, PartialEq, Eq, derive_more::Display, derive_more::Debug, Hash,
)]
#[serde(transparent)]
#[display("<OBFUSCATED>")]
#[debug("{:?}", self.non_sensitive())]
pub struct BIP39Passphrase(pub String);

impl SafeToLog for BIP39Passphrase {
    /// Logs the word count and FactorSourceID o
    fn non_sensitive(&self) -> impl std::fmt::Debug {
        if self.0.is_empty() {
            "<EMPTY>"
        } else {
            "<NOT EMPTY>"
        }
    }
}

impl BIP39Passphrase {
    pub fn new(s: impl AsRef<str>) -> Self {
        Self(s.as_ref().to_string())
    }
}

impl HasPlaceholder for BIP39Passphrase {
    fn placeholder() -> Self {
        Self::new("radix")
    }

    fn placeholder_other() -> Self {
        Self::new("just imagine...")
    }
}

impl From<&str> for BIP39Passphrase {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl Default for BIP39Passphrase {
    /// A user may decide to protect their mnemonic with a passphrase. If a passphrase is not present,
    /// an empty string "" is used instead.
    ///
    /// https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki#from-mnemonic-to-seed
    fn default() -> Self {
        Self("".to_string())
    }
}

uniffi::custom_newtype!(BIP39Passphrase, String);

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn json_roundtrip() {
        let sut: BIP39Passphrase = "25th word".into();

        assert_json_value_eq_after_roundtrip(&sut, json!("25th word"));
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(&sut, json!("foobar"));
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", BIP39Passphrase::new("so secret")),
            "<NOT EMPTY>"
        );
        assert_eq!(format!("{}", BIP39Passphrase::default()), "<EMPTY>");
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", BIP39Passphrase::new("so secret")),
            "<OBFUSCATED>"
        );
        assert_eq!(format!("{:?}", BIP39Passphrase::default()), "<OBFUSCATED>");
    }
}
