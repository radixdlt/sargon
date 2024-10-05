use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{}", self.to_canonical_string())]
pub struct NonFungibleGlobalId {
    // N.B. we WANT This to be a `NonFungibleResourceAddress` type, alas, it
    // cannot, since that validation does not happen part of Engine, so it is
    // possible (maybe even likely) that some Non Fungible tokens have addresses
    // which are "fungible" (i.e. entity type `GlobalFungibleResourceManager`
    // instead of `GlobalNonFungibleResourceManager`).
    //
    // For more info see slack:
    // https://rdxworks.slack.com/archives/C01HK4QFXNY/p1709633826502809?thread_ts=1709633374.199459&channel=C01HK4QFXNY&message_ts=1709633826.502809
    pub(crate) resource_address: ResourceAddress,
    pub(crate) non_fungible_local_id: NonFungibleLocalId,
}

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
