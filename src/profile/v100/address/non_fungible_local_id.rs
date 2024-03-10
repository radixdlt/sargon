use crate::prelude::*;

use radix_engine_common::prelude::BytesNonFungibleLocalId as ScryptoBytesNonFungibleLocalId;
use radix_engine_common::prelude::NonFungibleLocalId as ScryptoNonFungibleLocalId;
use radix_engine_common::prelude::StringNonFungibleLocalId as ScryptoStringNonFungibleLocalId;
use radix_engine_toolkit_json::models::common::SerializableNonFungibleLocalId as RetNonFungibleLocalId;

#[derive(Clone, Debug, Hash, Ord, PartialOrd, PartialEq, Eq, uniffi::Enum)]
pub enum NonFungibleLocalId {
    /// Unsigned integers, up to u64.
    ///
    /// Create using `NonFungibleLocalId::integer(...)`.
    Integer { value: u64 },

    /// String matching `[_0-9a-zA-Z]{1,64}`.
    ///
    /// Create using `NonFungibleLocalId::string(...).unwrap()`.
    Str { value: NonFungibleLocalIdString },

    /// Bytes, of length between 1 and 64.
    ///
    /// Create using `NonFungibleLocalId::bytes(...).unwrap()`.
    Bytes { value: NonEmptyMax64Bytes },

    /// RUID, v4, variant 1, big endian. See https://www.rfc-editor.org/rfc/rfc4122
    ///
    /// Create using `NonFungibleLocalId::ruid(...).unwrap()`.
    Ruid { value: Exactly32Bytes },
}

impl NonFungibleLocalId {
    pub fn integer(i: u64) -> Self {
        Self::Integer { value: i }
    }

    pub fn string(id: impl AsRef<str>) -> Result<Self> {
        id.as_ref()
            .parse::<NonFungibleLocalIdString>()
            .map(|value| Self::Str { value })
    }

    pub fn ruid(bytes: impl AsRef<[u8]>) -> Result<Self> {
        Exactly32Bytes::try_from(bytes.as_ref())
            .map(|value| Self::Ruid { value })
    }

    pub fn bytes(bytes: impl AsRef<[u8]>) -> Result<Self> {
        NonEmptyMax64Bytes::try_from(bytes.as_ref())
            .map(|value| Self::Bytes { value })
    }
}

#[uniffi::export]
pub fn non_fungible_local_id_as_str(id: NonFungibleLocalId) -> String {
    id.to_string()
}

#[uniffi::export]
pub fn new_non_fungible_local_id_int(value: u64) -> NonFungibleLocalId {
    NonFungibleLocalId::integer(value)
}

#[uniffi::export]
pub fn new_non_fungible_local_id_string(
    string: String,
) -> Result<NonFungibleLocalId> {
    NonFungibleLocalId::string(string)
}

#[uniffi::export]
pub fn new_non_fungible_local_id_bytes(
    bytes: BagOfBytes,
) -> Result<NonFungibleLocalId> {
    NonFungibleLocalId::bytes(bytes)
}

#[uniffi::export]
pub fn new_non_fungible_local_id_ruid(
    bytes: BagOfBytes,
) -> Result<NonFungibleLocalId> {
    NonFungibleLocalId::ruid(bytes)
}

impl NonFungibleLocalId {
    fn scrypto(&self) -> ScryptoNonFungibleLocalId {
        self.clone().into()
    }
}

impl std::fmt::Display for NonFungibleLocalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.scrypto())
    }
}

impl From<ScryptoBytesNonFungibleLocalId> for NonEmptyMax64Bytes {
    fn from(value: ScryptoBytesNonFungibleLocalId) -> Self {
        Self::try_from(value.value()).expect("Should not be possible, since ScryptoBytesNonFungibleLocalId have validated length")
    }
}

impl From<ScryptoNonFungibleLocalId> for NonFungibleLocalId {
    fn from(value: ScryptoNonFungibleLocalId) -> Self {
        match value {
            ScryptoNonFungibleLocalId::String(value) => Self::Str {
                value: value.into(),
            },
            ScryptoNonFungibleLocalId::Integer(value) => Self::Integer {
                value: value.value(),
            },
            ScryptoNonFungibleLocalId::Bytes(value) => Self::Bytes {
                value: value.into(),
            },
            ScryptoNonFungibleLocalId::RUID(value) => Self::Ruid {
                value: value.value().into(),
            },
        }
    }
}

impl From<NonFungibleLocalId> for ScryptoNonFungibleLocalId {
    fn from(value: NonFungibleLocalId) -> Self {
        match value {
            NonFungibleLocalId::Str { value } => Self::String(value.into()),
            NonFungibleLocalId::Bytes { value } => Self::bytes(value.to_vec()).expect("Should always be able to create Scrypto NonFungibleLocalId from bytes."),
            NonFungibleLocalId::Ruid { value } => Self::ruid(value.bytes()),
            NonFungibleLocalId::Integer { value } => Self::integer(value),
        }
    }
}

impl FromStr for NonFungibleLocalId {
    type Err = <ScryptoNonFungibleLocalId as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        ScryptoNonFungibleLocalId::from_str(s).map(Into::into)
    }
}

impl HasSampleValues for NonFungibleLocalId {
    fn sample() -> Self {
        Self::ruid(Exactly32Bytes::sample_dead()).unwrap()
    }

    fn sample_other() -> Self {
        Self::string("foobar").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use radix_engine_common::data::scrypto::model::{
        BytesNonFungibleLocalId as ScryptoBytesNonFungibleLocalId,
        IntegerNonFungibleLocalId as ScryptoIntegerNonFungibleLocalId,
        RUIDNonFungibleLocalId as ScryptoRUIDNonFungibleLocalId,
        StringNonFungibleLocalId as ScryptoStringNonFungibleLocalId,
    };

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleLocalId;

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
    fn from_str_ok() {
        assert_eq!(
            "<value>".parse::<SUT>().unwrap(),
            SUT::string("value").unwrap()
        );
    }

    #[test]
    fn display_integer() {
        assert_eq!(format!("{}", SUT::integer(1234)), "#1234#");
    }

    #[test]
    fn display_str() {
        assert_eq!(format!("{}", SUT::string("foo").unwrap()), "<foo>");
    }

    #[test]
    fn display_ruid() {
        assert_eq!(
            format!(
                "{}",
                SUT::ruid(
                    hex_decode("deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210").unwrap()
                ).unwrap()
            ),
            "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}"
        );
    }

    #[test]
    fn display_bytes() {
        assert_eq!(format!("{}", SUT::bytes([0xde, 0xad]).unwrap()), "[dead]");
    }

    #[test]
    fn from_str_err() {
        assert!(SUT::from_str("no_angle_brackets").is_err());
    }

    #[test]
    fn invalid_local_id_string() {
        assert_eq!(
            SUT::string(""),
            Err::<SUT, _>(CommonError::InvalidNonFungibleLocalIDString)
        );
    }

    #[test]
    fn from_native_ruid() {
        let bytes = Exactly32Bytes::sample_dead();
        let value = SUT::ruid(bytes.clone()).unwrap();
        let scrypto = ScryptoNonFungibleLocalId::RUID(
            ScryptoRUIDNonFungibleLocalId::new(bytes.clone().bytes()),
        );
        assert_eq!(value.clone(), scrypto.clone().into());
        assert_eq!(
            ScryptoNonFungibleLocalId::from(value.clone()),
            scrypto.clone()
        );
        assert_eq!(
            SUT::from_str(value.clone().to_string().as_str()),
            Ok(value)
        );
    }

    fn test_from_bytes<const N: usize>() {
        let bytes = [0xab; N];
        let value = SUT::bytes(bytes).unwrap();
        let scrypto = ScryptoNonFungibleLocalId::Bytes(
            ScryptoBytesNonFungibleLocalId::new(bytes.clone().to_vec())
                .unwrap(),
        );

        assert_eq!(
            ScryptoNonFungibleLocalId::from(value.clone()),
            scrypto.clone()
        );
        assert_eq!(
            SUT::from_str(value.clone().to_string().as_str()),
            Ok(value)
        );
    }

    #[test]
    fn from_bytes() {
        assert_eq!(SUT::bytes([]), Err(CommonError::BytesEmpty));
        test_from_bytes::<1>();
        test_from_bytes::<2>();
        test_from_bytes::<63>();
        test_from_bytes::<64>();
    }

    #[test]
    fn from_native_str() {
        let value = SUT::string("test").unwrap();
        let scrypto = ScryptoNonFungibleLocalId::String(
            ScryptoStringNonFungibleLocalId::new("test").unwrap(),
        );
        assert_eq!(value.clone(), scrypto.clone().into());
        assert_eq!(
            SUT::from_str(value.clone().to_string().as_str()),
            Ok(value)
        );
    }

    #[test]
    fn from_native_integer() {
        let non_native = SUT::Integer { value: 1234 };
        let native = ScryptoNonFungibleLocalId::Integer(
            ScryptoIntegerNonFungibleLocalId::new(1234),
        );
        assert_eq!(non_native.clone(), native.clone().into());
        assert_eq!(
            SUT::from_str(non_native.clone().to_string().as_str()),
            Ok(non_native)
        );
    }

    #[test]
    fn display_rui() {
        assert_eq!(
            non_fungible_local_id_as_str(SUT::ruid(
                hex_decode("deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210").unwrap()
            ).unwrap()),
            "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}"
        );
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleLocalId;

    #[test]
    fn integer() {
        assert_eq!(
            new_non_fungible_local_id_int(1337),
            SUT::Integer { value: 1337 }
        );
    }

    #[test]
    fn bytes() {
        let s = "dead";
        let b = NonEmptyMax64Bytes::from_hex(s).unwrap();
        assert_eq!(
            new_non_fungible_local_id_bytes(BagOfBytes::from_hex(s).unwrap())
                .unwrap(),
            SUT::Bytes { value: b }
        );
    }

    #[test]
    fn ruid() {
        assert_eq!(new_non_fungible_local_id_ruid(BagOfBytes::sample_aced()).unwrap().to_string(), "{acedacedacedaced-acedacedacedaced-acedacedacedaced-acedacedacedaced}");
    }

    #[test]
    fn string() {
        assert_eq!(
            new_non_fungible_local_id_string("foo".to_owned())
                .unwrap()
                .to_string(),
            "<foo>"
        );
    }
}
