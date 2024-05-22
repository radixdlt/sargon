use crate::prelude::*;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionHandshakeResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub public_key: PublicKey,
}

impl SessionHandshakeResponse {
    pub fn new(public_key: impl Into<PublicKey>) -> Self {
        Self {
            public_key: public_key.into(),
        }
    }
}

impl HasSampleValues for SessionHandshakeResponse {
    fn sample() -> Self {
        Self::new(PublicKey::sample())
    }

    fn sample_other() -> Self {
        Self::new(PublicKey::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SessionHandshakeResponse;

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
        let original = SUT::sample();
        let json = format!(
            r#"
            {{
                "publicKey":"{}"
            }}
            "#,
            original.public_key.to_hex()
        );

        assert_eq_after_json_roundtrip(&original, &json);
    }
}
