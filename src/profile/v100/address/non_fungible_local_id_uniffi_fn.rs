use crate::prelude::*;

#[uniffi::export]
pub fn new_non_fungible_local_id_from_string(
    local_id: String,
) -> Result<NonFungibleLocalId> {
    NonFungibleLocalId::from_str(&local_id)
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
}
