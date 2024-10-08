use crate::prelude::*;
use sargon::NonFungibleLocalId as InternalNonFungibleLocalId;

#[derive(Clone, Debug, Hash, PartialEq, Eq, uniffi::Enum)]
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

impl From<InternalNonFungibleLocalId> for NonFungibleLocalId {
    fn from(value: InternalNonFungibleLocalId) -> Self {
        match value {
            InternalNonFungibleLocalId::Integer { value } => {
                NonFungibleLocalId::Integer { value }
            }
            InternalNonFungibleLocalId::Str { value } => {
                NonFungibleLocalId::Str {
                    value: value.into(),
                }
            }
            InternalNonFungibleLocalId::Bytes { value } => {
                NonFungibleLocalId::Bytes {
                    value: value.into(),
                }
            }
            InternalNonFungibleLocalId::Ruid { value } => {
                NonFungibleLocalId::Ruid {
                    value: value.into(),
                }
            }
        }
    }
}

impl Into<InternalNonFungibleLocalId> for NonFungibleLocalId {
    fn into(self) -> InternalNonFungibleLocalId {
        match self {
            NonFungibleLocalId::Integer { value } => {
                InternalNonFungibleLocalId::Integer { value }
            }
            NonFungibleLocalId::Str { value } => {
                InternalNonFungibleLocalId::Str { value: value.into() }
            }
            NonFungibleLocalId::Bytes { value } => {
                InternalNonFungibleLocalId::Bytes { value: value.into() }
            }
            NonFungibleLocalId::Ruid { value } => {
                InternalNonFungibleLocalId::Ruid { value: value.into() }
            }
        }
    }
}

#[uniffi::export]
pub fn new_non_fungible_local_id_from_string(
    local_id: String,
) -> Result<NonFungibleLocalId> {
    InternalNonFungibleLocalId::from_str(&local_id).map_result()
}

#[uniffi::export]
pub fn non_fungible_local_id_as_str(id: NonFungibleLocalId) -> String {
    id.into_internal().to_string()
}

#[uniffi::export]
pub fn non_fungible_local_id_to_user_facing_string(
    id: &NonFungibleLocalId,
) -> String {
    id.into_internal().to_user_facing_string()
}

#[uniffi::export]
pub fn non_fungible_local_id_formatted(
    id: &NonFungibleLocalId,
    format: AddressFormat,
) -> String {
    id.into_internal().formatted(format.into())
}

#[uniffi::export]
pub fn new_non_fungible_local_id_int(value: u64) -> NonFungibleLocalId {
    NonFungibleLocalId::integer(value)
}

#[uniffi::export]
pub fn new_non_fungible_local_id_string(
    string: String,
) -> Result<NonFungibleLocalId> {
    InternalNonFungibleLocalId::string(string).map_result()
}

#[uniffi::export]
pub fn new_non_fungible_local_id_bytes(
    bytes: BagOfBytes,
) -> Result<NonFungibleLocalId> {
    InternalNonFungibleLocalId::bytes(bytes.into()).map_result()
}

#[uniffi::export]
pub fn new_non_fungible_local_id_ruid(
    bytes: BagOfBytes,
) -> Result<NonFungibleLocalId> {
    InternalNonFungibleLocalId::ruid(bytes.into()).map_result()
}

#[uniffi::export]
pub fn new_non_fungible_local_id_sample() -> NonFungibleLocalId {
    InternalNonFungibleLocalId::sample().into()
}

#[uniffi::export]
pub fn new_non_fungible_local_id_sample_other() -> NonFungibleLocalId {
    InternalNonFungibleLocalId::sample_other().into()
}

#[uniffi::export]
pub fn new_non_fungible_local_id_random() -> NonFungibleLocalId {
    InternalNonFungibleLocalId::random().into()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleLocalId;

    #[test]
    fn from_integer_string() {
        assert_eq!(
            new_non_fungible_local_id_from_string("#1#".to_owned()).unwrap(),
            SUT::integer(1)
        );
    }

    #[test]
    fn from_string_string() {
        let expected = SUT::string("foo").unwrap();
        assert_eq!(
            new_non_fungible_local_id_from_string(expected.to_string())
                .unwrap(),
            expected
        );
    }

    #[test]
    fn from_ruid_string() {
        let expected = SUT::ruid(Exactly32Bytes::sample_dead()).unwrap();
        assert_eq!(
            new_non_fungible_local_id_from_string(expected.to_string())
                .unwrap(),
            expected
        );
    }

    #[test]
    fn from_bytes_string() {
        let expected = SUT::bytes(Exactly32Bytes::sample_dead()).unwrap();
        assert_eq!(
            new_non_fungible_local_id_from_string(expected.to_string())
                .unwrap(),
            expected
        );
    }

    #[test]
    fn from_invalid_string() {
        assert_eq!(
            new_non_fungible_local_id_from_string("foo".to_owned()),
            Err::<SUT, _>(CommonError::InvalidNonFungibleLocalIDString)
        );
    }

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

    #[test]
    fn test_samples() {
        assert_eq!(
            NonFungibleLocalId::sample(),
            new_non_fungible_local_id_sample(),
        );

        assert_eq!(
            NonFungibleLocalId::sample_other(),
            new_non_fungible_local_id_sample_other(),
        );
    }

    #[test]
    fn to_user_facing_str() {
        let sut = SUT::sample();
        assert_eq!(
            non_fungible_local_id_to_user_facing_string(&sut),
            sut.to_user_facing_string()
        )
    }

    #[test]
    fn formatted_default() {
        let sut = SUT::sample();
        assert_eq!(
            non_fungible_local_id_formatted(&sut, AddressFormat::Default),
            sut.formatted(AddressFormat::Default)
        )
    }

    #[test]
    fn test_new_non_fungible_local_id_random() {
        let mut set: HashSet<SUT> = HashSet::new();
        let n = 100;
        for _ in 0..n {
            set.insert(new_non_fungible_local_id_random());
        }
        assert_eq!(set.len(), n);
    }
}
