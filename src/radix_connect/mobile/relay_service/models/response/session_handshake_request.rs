use crate::prelude::*;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionHandshakeRequest {
    #[serde_as(as = "DisplayFromStr")]
    pub public_key: KeyAgreementPublicKey,
}

impl SessionHandshakeRequest {
    pub fn new(public_key: impl Into<KeyAgreementPublicKey>) -> Self {
        Self {
            public_key: public_key.into(),
        }
    }
}

impl HasSampleValues for SessionHandshakeRequest {
    fn sample() -> Self {
        Self::new(KeyAgreementPublicKey::sample())
    }

    fn sample_other() -> Self {
        Self::new(KeyAgreementPublicKey::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type SUT = SessionHandshakeRequest;

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
