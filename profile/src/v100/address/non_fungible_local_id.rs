// use radix_engine_common::data::scrypto::model::NonFungibleLocalId as NativeNonFungibleLocalId;
use radix_engine_common::data::scrypto::model::NonFungibleLocalId as NativeNonFungibleLocalId;

#[derive(Clone, Debug, uniffi::Enum, Hash, PartialEq, Eq)]
pub enum NonFungibleLocalId {
    Integer { value: u64 },
    Str { value: String },
    Bytes { value: Vec<u8> },
    Ruid { value: Vec<u8> },
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
            NonFungibleLocalId::Ruid { value } => {
                value
                    .try_into()
                    .map(Self::ruid)
                    .map_err(|value| Self::Error::InvalidLength {
                        expected: 32,
                        actual: value.len(),
                        data: value,
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
