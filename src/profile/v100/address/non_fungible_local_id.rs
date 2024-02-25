use std::io::Read;

use crate::prelude::*;

use radix_engine_common::prelude::NonFungibleLocalId as ScryptoNonFungibleLocalId;
use radix_engine_common::prelude::StringNonFungibleLocalId as ScryptoStringNonFungibleLocalId;
use radix_engine_toolkit_json::models::common::SerializableNonFungibleLocalId as RetNonFungibleLocalId;

#[derive(Clone, Debug, Hash, PartialEq, Eq, uniffi::Enum)]
pub enum NonFungibleLocalId {
    Integer { value: u64 },
    Str { value: NonFungibleLocalIdString },
    Bytes { value: BagOfBytes },
    Ruid { value: Hex32Bytes },
}

impl NonFungibleLocalId {
    pub fn string(id: impl AsRef<str>) -> Result<Self> {
        id.as_ref()
            .parse::<NonFungibleLocalIdString>()
            .map(|value| Self::Str { value })
    }
    pub fn ruid(bytes: impl AsRef<[u8]>) -> Result<Self> {
        Hex32Bytes::try_from(bytes.as_ref()).map(|value| Self::Ruid { value })
    }
}

#[uniffi::export]
pub fn non_fungible_local_id_as_str(id: NonFungibleLocalId) -> String {
    id.to_string()
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

#[derive(
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[debug("{}", self.to_string())]
#[display("{}", self.secret_magic.value().to_owned())]
pub struct NonFungibleLocalIdString {
    secret_magic: ScryptoStringNonFungibleLocalId,
}
impl FromStr for NonFungibleLocalIdString {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        scrypto_string_non_fungible_local_id(s).map(|i| i.into())
    }
}

impl From<ScryptoStringNonFungibleLocalId> for NonFungibleLocalIdString {
    fn from(value: ScryptoStringNonFungibleLocalId) -> Self {
        Self {
            secret_magic: value,
        }
    }
}
impl From<NonFungibleLocalIdString> for ScryptoStringNonFungibleLocalId {
    fn from(value: NonFungibleLocalIdString) -> Self {
        value.secret_magic
    }
}

fn scrypto_string_non_fungible_local_id(
    string: impl AsRef<str>,
) -> Result<ScryptoStringNonFungibleLocalId> {
    ScryptoStringNonFungibleLocalId::new(string).map_err(|e| {
        error!("{:?}", e);
        CommonError::InvalidNonFungibleLocalIDString
    })
}

impl crate::UniffiCustomTypeConverter for ScryptoStringNonFungibleLocalId {
    type Builtin = String;

    #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        scrypto_string_non_fungible_local_id(val).map_err(|e| e.into())
    }

    #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.value().to_owned()
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
                value: value.value().to_vec().into(),
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

    #[test]
    fn from_str_ok() {
        assert_eq!(
            "<value>".parse::<NonFungibleLocalId>().unwrap(),
            NonFungibleLocalId::string("value").unwrap()
        );
    }

    #[test]
    fn display_integer() {
        assert_eq!(
            format!("{}", NonFungibleLocalId::Integer { value: 1234 }),
            "#1234#"
        );
    }

    #[test]
    fn display_str() {
        assert_eq!(
            format!("{}", NonFungibleLocalId::string("foo").unwrap()),
            "<foo>"
        );
    }

    #[test]
    fn display_ruid() {
        assert_eq!(
            format!(
                "{}",
                NonFungibleLocalId::ruid(
                    hex_decode("deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210").unwrap()
                ).unwrap()
            ),
            "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}"
        );
    }

    #[test]
    fn display_bytes() {
        assert_eq!(
            format!(
                "{}",
                NonFungibleLocalId::Bytes {
                    value: vec![0xde, 0xad].into()
                }
            ),
            "[dead]"
        );
    }

    #[test]
    fn from_str_err() {
        assert!(NonFungibleLocalId::from_str("no_angle_brackets").is_err());
    }

    #[test]
    fn invalid_local_id_string() {
        assert_eq!(
            NonFungibleLocalId::string(""),
            Err::<NonFungibleLocalId, _>(
                CommonError::InvalidNonFungibleLocalIDString
            )
        );
    }

    #[test]
    fn from_native_ruid() {
        let bytes = Hex32Bytes::placeholder_dead();
        let value = NonFungibleLocalId::ruid(bytes.clone()).unwrap();
        let scrypto = ScryptoNonFungibleLocalId::RUID(
            ScryptoRUIDNonFungibleLocalId::new(bytes.clone().bytes()),
        );
        assert_eq!(value.clone(), scrypto.clone().into());
        assert_eq!(value.clone().try_into(), Ok(scrypto.clone()));
        assert_eq!(
            NonFungibleLocalId::from_str(value.clone().to_string().as_str()),
            Ok(value)
        );
    }

    #[test]
    fn from_native_bytes() {
        let bytes = [0xab; 64];
        let non_native = NonFungibleLocalId::Bytes {
            value: bytes.clone().to_vec().into(),
        };
        let native = ScryptoNonFungibleLocalId::Bytes(
            ScryptoBytesNonFungibleLocalId::new(bytes.clone().to_vec())
                .unwrap(),
        );
        assert_eq!(non_native.clone(), native.clone().into());
        assert_eq!(non_native.clone().try_into(), Ok(native.clone()));
        assert_eq!(
            NonFungibleLocalId::from_str(
                non_native.clone().to_string().as_str()
            ),
            Ok(non_native)
        );
    }

    #[test]
    fn from_native_str() {
        let value = NonFungibleLocalId::string("test").unwrap();
        let scrypto = ScryptoNonFungibleLocalId::String(
            ScryptoStringNonFungibleLocalId::new("test").unwrap(),
        );
        assert_eq!(value.clone(), scrypto.clone().into());
        assert_eq!(value.clone().try_into(), Ok(scrypto.clone()));
        assert_eq!(
            NonFungibleLocalId::from_str(value.clone().to_string().as_str()),
            Ok(value)
        );
    }

    #[test]
    fn from_native_integer() {
        let non_native = NonFungibleLocalId::Integer { value: 1234 };
        let native = ScryptoNonFungibleLocalId::Integer(
            ScryptoIntegerNonFungibleLocalId::new(1234),
        );
        assert_eq!(non_native.clone(), native.clone().into());
        assert_eq!(non_native.clone().try_into(), Ok(native.clone()));
        assert_eq!(
            NonFungibleLocalId::from_str(
                non_native.clone().to_string().as_str()
            ),
            Ok(non_native)
        );
    }

    #[test]
    fn display_rui() {
        assert_eq!(
            non_fungible_local_id_as_str(NonFungibleLocalId::ruid(
                hex_decode("deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210").unwrap()
            ).unwrap()),
            "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}"
        );
    }
}
