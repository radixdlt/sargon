use crate::prelude::*;

/// A BIP39 passphrase, which required but when not used by user, the Default value will be use (empty string),
/// as per BIP39 standard.
#[derive(
    Zeroize,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    Hash,
)]
#[serde(transparent)]
#[display("<OBFUSCATED>")]
#[debug("{:?}", self.partially_obfuscated_string())]
pub struct BIP39Passphrase(pub String);

impl BIP39Passphrase {
    pub fn partially_obfuscated_string(&self) -> String {
        if self.0.is_empty() {
            "<EMPTY>"
        } else {
            "<NOT EMPTY>"
        }
        .to_string()
    }
}

impl BIP39Passphrase {
    pub fn new(s: impl AsRef<str>) -> Self {
        Self(s.as_ref().to_string())
    }
}

impl HasSampleValues for BIP39Passphrase {
    fn sample() -> Self {
        Self::new("radix")
    }

    fn sample_other() -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BIP39Passphrase;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn json_roundtrip() {
        let sut: SUT = "25th word".into();

        assert_json_value_eq_after_roundtrip(&sut, json!("25th word"));
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(&sut, json!("foobar"));
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::new("so secret")),
            format!("{:?}", "<NOT EMPTY>")
        );
        assert_eq!(format!("{:?}", SUT::default()), format!("{:?}", "<EMPTY>"));
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::new("so secret")), "<OBFUSCATED>");
        assert_eq!(format!("{}", SUT::default()), "<OBFUSCATED>");
    }

    #[test]
    fn zeroize() {
        let mut sut = SUT::sample();
        sut.zeroize();
        assert_eq!(sut.0, "");
    }
}
