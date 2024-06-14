use crate::prelude::*;

const ROLA_PAYLOAD_PREFIX: u8 = 0x52;

#[derive(Debug, Eq, PartialEq, uniffi::Record)]
pub struct SignAuthChallengeRequest {
    pub challenge: Exactly32Bytes,
    pub origin: Url,
    pub dapp_definition_address: DappDefinitionAddress
}

impl SignAuthChallengeRequest {
    pub fn new(
        challenge: impl Into<Exactly32Bytes>,
        origin: impl Into<Url>,
        dapp_definition_address: DappDefinitionAddress
    ) -> Self {
        Self {
            challenge: challenge.into(),
            origin: origin.into(),
            dapp_definition_address
        }
    }

    pub fn data_to_sign(&self) -> BagOfBytes {
        let mut data = Vec::new();
        data.push(ROLA_PAYLOAD_PREFIX);        
        data.extend_from_slice(&exactly_32_bytes_to_bytes(&self.challenge));
        data.push(dapp_definition_address.len() as u8);
        data.extend_from_slice(&self.origin.as_str().as_bytes());
        data.extend_from_slice(&self.dapp_definition_address.);
        BagOfBytes::from(data)
    }
}

impl HasSampleValues for SignAuthChallengeRequest {
    fn sample() -> Self {
        Self::new(
            Exactly32Bytes::sample(),
            Url::parse("https://example.com").unwrap(),
            DappDefinitionAddress::sample()
        )
    }

    fn sample_other() -> Self {
        Self::new(
            Exactly32Bytes::sample_other(),
            Url::parse("https://example.org").unwrap(),
            DappDefinitionAddress::sample_other()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignAuthChallengeRequest;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}