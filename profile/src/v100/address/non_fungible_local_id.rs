use radix_engine_common::data::scrypto::model::NonFungibleLocalId as NativeNonFungibleLocalId;

#[derive(Clone, Debug, uniffi::Enum, Hash, PartialEq, Eq)]
pub enum NonFungibleLocalId {
    Integer { value: u64 },
    Str { value: String },
    Bytes { value: Vec<u8> },
    Ruid { value: Vec<u8> },
}

impl NonFungibleLocalId {
    pub fn to_string(&self) -> String {
        let native = NativeNonFungibleLocalId::try_from(self.clone()).unwrap();
        native.to_string()
    }
}

impl From<NativeNonFungibleLocalId> for NonFungibleLocalId {
    fn from(value: NativeNonFungibleLocalId) -> Self {
        match value {
            NativeNonFungibleLocalId::String(value) => Self::Str {
                value: value.value().to_owned(),
            },
            NativeNonFungibleLocalId::Integer(value) => Self::Integer {
                value: value.value(),
            },
            NativeNonFungibleLocalId::Bytes(value) => Self::Bytes {
                value: value.value().to_vec(),
            },
            NativeNonFungibleLocalId::RUID(value) => Self::Ruid {
                value: value.value().to_vec(),
            },
        }
    }
}

impl TryFrom<NonFungibleLocalId> for NativeNonFungibleLocalId {
    type Error = crate::CommonError;

    fn try_from(value: NonFungibleLocalId) -> Result<Self, crate::CommonError> {
        match value {
            NonFungibleLocalId::Str { value } => {
                Self::string(value).map_err(|_| Self::Error::InvalidNonFungibleLocalIDString)
            }
            NonFungibleLocalId::Bytes { value } => {
                Self::bytes(value).map_err(|_| Self::Error::InvalidNonFungibleLocalIDBytes)
            }
            NonFungibleLocalId::Ruid { value } => value
                .try_into()
                .map(Self::ruid)
                .map_err(|value| Self::Error::InvalidLength {
                    expected: 32,
                    actual: value.len(),
                    data: value,
                }),
            NonFungibleLocalId::Integer { value } => Ok(Self::integer(value)),
        }
    }
}

impl std::str::FromStr for NonFungibleLocalId {
    type Err = <NativeNonFungibleLocalId as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        NativeNonFungibleLocalId::from_str(s).map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use crate::{CommonError, Hex32Bytes};

    use super::NonFungibleLocalId;
    use radix_engine_common::data::scrypto::model::{
        BytesNonFungibleLocalId, IntegerNonFungibleLocalId,
        NonFungibleLocalId as NativeNonFungibleLocalId, RUIDNonFungibleLocalId,
        StringNonFungibleLocalId,
    };
    use std::str::FromStr;

    #[test]
    fn from_str_ok() {
        assert_eq!(
            NonFungibleLocalId::from_str("<value>"),
            Ok(NonFungibleLocalId::Str {
                value: "value".to_string()
            })
        );
    }

    #[test]
    fn from_str_err() {
        assert!(NonFungibleLocalId::from_str("no_angle_brackets").is_err());
    }

    #[test]
    fn from_native_ruid() {
        let bytes = Hex32Bytes::placeholder_dead()
            .bytes()
            .to_owned();
        let non_native = NonFungibleLocalId::Ruid {
            value: bytes.clone().to_vec(),
        };
        let native = NativeNonFungibleLocalId::RUID(RUIDNonFungibleLocalId::new(bytes.clone()));
        assert_eq!(non_native.clone(), native.clone().into());
        assert_eq!(non_native.clone().try_into(), Ok(native.clone()));
        assert_eq!(
            NonFungibleLocalId::from_str(
                non_native
                    .clone()
                    .to_string()
                    .as_str()
            ),
            Ok(non_native)
        );
    }

    #[test]
    fn from_native_bytes() {
        let bytes = [0xab; 64];
        let non_native = NonFungibleLocalId::Bytes {
            value: bytes.clone().to_vec(),
        };
        let native = NativeNonFungibleLocalId::Bytes(
            BytesNonFungibleLocalId::new(bytes.clone().to_vec()).unwrap(),
        );
        assert_eq!(non_native.clone(), native.clone().into());
        assert_eq!(non_native.clone().try_into(), Ok(native.clone()));
        assert_eq!(
            NonFungibleLocalId::from_str(
                non_native
                    .clone()
                    .to_string()
                    .as_str()
            ),
            Ok(non_native)
        );
    }

    #[test]
    fn from_native_str() {
        let non_native = NonFungibleLocalId::Str {
            value: "test".to_string(),
        };
        let native =
            NativeNonFungibleLocalId::String(StringNonFungibleLocalId::new("test").unwrap());
        assert_eq!(non_native.clone(), native.clone().into());
        assert_eq!(non_native.clone().try_into(), Ok(native.clone()));
        assert_eq!(
            NonFungibleLocalId::from_str(
                non_native
                    .clone()
                    .to_string()
                    .as_str()
            ),
            Ok(non_native)
        );
    }

    #[test]
    fn from_native_integer() {
        let non_native = NonFungibleLocalId::Integer { value: 1234 };
        let native = NativeNonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1234));
        assert_eq!(non_native.clone(), native.clone().into());
        assert_eq!(non_native.clone().try_into(), Ok(native.clone()));
        assert_eq!(
            NonFungibleLocalId::from_str(
                non_native
                    .clone()
                    .to_string()
                    .as_str()
            ),
            Ok(non_native)
        );
    }

    #[test]
    fn to_native_from_invalid_byte_count_throws() {
        let invalid = NonFungibleLocalId::Ruid { value: Vec::new() };
        assert_eq!(
            NativeNonFungibleLocalId::try_from(invalid),
            Err(CommonError::InvalidLength {
                expected: 32,
                actual: 0,
                data: Vec::new()
            })
        );
    }
}
