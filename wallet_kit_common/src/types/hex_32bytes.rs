use std::str::FromStr;

use serde::{de, Deserializer, Serialize, Serializer};

use crate::error::Error;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hex32Bytes([u8; 32]);

impl ToString for Hex32Bytes {
    fn to_string(&self) -> String {
        hex::encode(self.0)
    }
}

impl FromStr for Hex32Bytes {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hex::decode(s)
            .map_err(|_| Error::StringNotHex)
            .and_then(|v| Self::from_vec(v))
    }
}

impl Hex32Bytes {
    pub fn placeholder() -> Self {
        Self::from_str("deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
            .expect("Deadbeef")
    }

    pub fn to_vec(&self) -> Vec<u8> {
        Vec::from(self.bytes().clone())
    }

    pub fn bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        Self(bytes.clone())
    }

    pub fn from_vec(bytes: Vec<u8>) -> Result<Self, Error> {
        bytes
            .try_into()
            .map(|b| Self(b))
            .map_err(|_| Error::InvalidByteCountExpected32)
    }

    pub fn from_hex(s: &str) -> Result<Self, Error> {
        Self::from_str(s)
    }
}

impl Serialize for Hex32Bytes {
    /// Serializes this `AccountAddress` into its bech32 address string as JSON.
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Hex32Bytes {
    /// Tries to deserializes a JSON string as a bech32 address into an `AccountAddress`.
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Hex32Bytes, D::Error> {
        let s = String::deserialize(d)?;
        Hex32Bytes::from_hex(&s).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::Hex32Bytes;

    #[test]
    fn from_string_roundtrip() {
        let str = "0000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(Hex32Bytes::from_hex(str).unwrap().to_string(), str);
    }

    #[test]
    fn from_bytes_roundtrip() {
        let bytes = [0u8; 32];
        assert_eq!(Hex32Bytes::from_bytes(&bytes).bytes(), &bytes);
    }

    #[test]
    fn from_vec_roundtrip() {
        let vec = Vec::from([0u8; 32]);
        assert_eq!(Hex32Bytes::from_vec(vec.clone()).unwrap().to_vec(), vec);
    }
}
