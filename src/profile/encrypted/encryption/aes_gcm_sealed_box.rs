use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct AesGcmSealedBox {
    /// Nonce is 12 bytes
    pub(super) nonce: Exactly12Bytes,

    /// Auth tag and encrypted payload
    pub(super) cipher_text: Vec<u8>,
}

impl AesGcmSealedBox {
    pub const AUTH_TAG_LEN: usize = 16;
    pub const NONCE_LEN: usize = 12;
    pub const LOWER_BOUND_LEN: usize = Self::AUTH_TAG_LEN + Self::NONCE_LEN + 1; // at least 1 byte cipher. VERY much LOWER bound

    pub(super) fn combined(self) -> Vec<u8> {
        let mut combined = Vec::<u8>::new();
        let mut nonce = self.nonce.to_vec();
        let mut cipher_text = self.cipher_text;
        combined.append(&mut nonce);
        combined.append(&mut cipher_text);
        assert!(combined.len() >= Self::LOWER_BOUND_LEN);
        combined
    }
}

impl TryFrom<Vec<u8>> for AesGcmSealedBox {
    type Error = CommonError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() < Self::LOWER_BOUND_LEN {
            return Err(CommonError::InvalidAESBytesTooShort {
                expected_at_least: Self::LOWER_BOUND_LEN as u64,
                found: value.len() as u64,
            });
        }

        let mut bytes = value;
        let nonce_bytes = bytes.drain(..Self::NONCE_LEN).collect_vec();
        let nonce = Exactly12Bytes::try_from(nonce_bytes).unwrap();
        Ok(Self {
            nonce,
            cipher_text: bytes,
        })
    }
}
