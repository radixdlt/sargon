use crate::prelude::*;

use radix_engine_common::data::scrypto::model::NonFungibleLocalId as NativeNonFungibleLocalId;

#[derive(Clone, Debug, Hash, PartialEq, Eq, uniffi::Enum)]
pub enum NonFungibleLocalId {
    Integer { value: u64 },
    Str { value: String },
    Bytes { value: BagOfBytes },
    Ruid { value: BagOfBytes },
}

#[uniffi::export]
pub fn non_fungible_local_id_to_string(id: NonFungibleLocalId) -> String {
    id.to_string()
}

impl NonFungibleLocalId {
    fn native(&self) -> NativeNonFungibleLocalId {
        NativeNonFungibleLocalId::try_from(self.clone()).unwrap()
    }
}

impl std::fmt::Display for NonFungibleLocalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.native())
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
                value: value.value().to_vec().into(),
            },
            NativeNonFungibleLocalId::RUID(value) => Self::Ruid {
                value: value.value().to_vec().into(),
            },
        }
    }
}

impl TryFrom<NonFungibleLocalId> for NativeNonFungibleLocalId {
    type Error = crate::CommonError;

    fn try_from(value: NonFungibleLocalId) -> Result<Self, crate::CommonError> {
        match value {
            NonFungibleLocalId::Str { value } => Self::string(value)
                .map_err(|_| Self::Error::InvalidNonFungibleLocalIDString),
            NonFungibleLocalId::Bytes { value } => Self::bytes(value.to_vec())
                .map_err(|_| Self::Error::InvalidNonFungibleLocalIDBytes),
            NonFungibleLocalId::Ruid { value } => {
                value.to_vec().try_into().map(Self::ruid).map_err(|value| {
                    CommonError::InvalidLength {
                        expected: 32,
                        found: value.len(),
                        data: value,
                    }
                })
            }
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
    use crate::prelude::*;

    use radix_engine_common::data::scrypto::model::{
        BytesNonFungibleLocalId, IntegerNonFungibleLocalId,
        NonFungibleLocalId as NativeNonFungibleLocalId, RUIDNonFungibleLocalId,
        StringNonFungibleLocalId,
    };

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
    fn display_integer() {
        assert_eq!(
            format!("{}", NonFungibleLocalId::Integer { value: 1234 }),
            "#1234#"
        );
    }

    #[test]
    fn display_str() {
        assert_eq!(
            format!(
                "{}",
                NonFungibleLocalId::Str {
                    value: "foo".to_owned()
                }
            ),
            "<foo>"
        );
    }

    #[test]
    fn display_ruid() {
        assert_eq!(
            format!(
                "{}",
                NonFungibleLocalId::Ruid {
                    value: hex_decode("deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210").unwrap().into()
                }
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
            NonFungibleLocalId::Str {
                value: "".to_string()
            }
            .try_into(),
            Err::<NativeNonFungibleLocalId, _>(
                CommonError::InvalidNonFungibleLocalIDString
            )
        );
    }

    #[test]
    fn invalid_local_id_bytes() {
        assert_eq!(
            NonFungibleLocalId::Bytes {
                value: BagOfBytes::new()
            }
            .try_into(),
            Err::<NativeNonFungibleLocalId, _>(
                CommonError::InvalidNonFungibleLocalIDBytes
            )
        );
    }

    #[test]
    fn from_native_ruid() {
        let bytes = Hex32Bytes::placeholder_dead().bytes().to_owned();
        let non_native = NonFungibleLocalId::Ruid {
            value: bytes.clone().to_vec().into(),
        };
        let native =
            NativeNonFungibleLocalId::RUID(RUIDNonFungibleLocalId::new(bytes));
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
    fn from_native_bytes() {
        let bytes = [0xab; 64];
        let non_native = NonFungibleLocalId::Bytes {
            value: bytes.clone().to_vec().into(),
        };
        let native = NativeNonFungibleLocalId::Bytes(
            BytesNonFungibleLocalId::new(bytes.clone().to_vec()).unwrap(),
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
        let non_native = NonFungibleLocalId::Str {
            value: "test".to_string(),
        };
        let native = NativeNonFungibleLocalId::String(
            StringNonFungibleLocalId::new("test").unwrap(),
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
    fn from_native_integer() {
        let non_native = NonFungibleLocalId::Integer { value: 1234 };
        let native = NativeNonFungibleLocalId::Integer(
            IntegerNonFungibleLocalId::new(1234),
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
    fn to_native_from_invalid_byte_count_throws() {
        let invalid = NonFungibleLocalId::Ruid {
            value: BagOfBytes::new(),
        };
        assert_eq!(
            NativeNonFungibleLocalId::try_from(invalid),
            Err(CommonError::InvalidLength {
                expected: 32,
                found: 0,
                data: Vec::new()
            })
        );
    }
}
