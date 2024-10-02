use crate::prelude::*;

#[uniffi::export]
pub fn new_non_fungible_global_id_from_string(
    string: String,
) -> Result<NonFungibleGlobalId> {
    NonFungibleGlobalId::from_str(&string)
}

#[uniffi::export]
pub fn new_non_fungible_global_id_sample() -> NonFungibleGlobalId {
    NonFungibleGlobalId::sample()
}

#[uniffi::export]
pub fn new_non_fungible_global_id_sample_other() -> NonFungibleGlobalId {
    NonFungibleGlobalId::sample_other()
}

#[uniffi::export]
pub fn non_fungible_global_id_to_string(
    global_id: &NonFungibleGlobalId,
) -> String {
    global_id.to_string()
}

#[uniffi::export]
pub fn non_fungible_global_id_formatted(
    global_id: &NonFungibleGlobalId,
    format: AddressFormat,
) -> String {
    global_id.formatted(format)
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleGlobalId;

    #[test]
    fn test_from_global_id() {
        let global_id = SUT::sample();

        assert_eq!(
            new_non_fungible_global_id_from_string(
                non_fungible_global_id_to_string(&global_id)
            )
            .unwrap(),
            global_id
        );
    }

    #[test]
    fn test_samples() {
        assert_eq!(SUT::sample(), new_non_fungible_global_id_sample());

        assert_eq!(
            SUT::sample_other(),
            new_non_fungible_global_id_sample_other()
        );
    }

    #[test]
    fn formatted_default() {
        let sut = SUT::sample();
        assert_eq!(
            non_fungible_global_id_formatted(&sut, AddressFormat::Default),
            sut.formatted(AddressFormat::Default)
        )
    }
}
