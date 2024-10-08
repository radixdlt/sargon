use crate::prelude::*;
use sargon::NonFungibleGlobalId as InternalNonFungibleGlobalId;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Record,
)]
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

impl From<InternalNonFungibleGlobalId> for NonFungibleGlobalId {
    fn from(value: InternalNonFungibleGlobalId) -> Self {
        Self {
            resource_address: value.resource_address.into(),
            non_fungible_local_id: value.non_fungible_local_id.into(),
        }
    }
}

impl Into<InternalNonFungibleGlobalId> for NonFungibleGlobalId {
    fn into(self) -> InternalNonFungibleGlobalId {
        InternalNonFungibleGlobalId {
            resource_address: self.resource_address.into(),
            non_fungible_local_id: self.non_fungible_local_id.into(),
        }
    }
}

pub trait IntoInternal<InternalType> {
    fn into_internal(self) -> InternalType;
}

impl<InternalType, Type> IntoInternal<InternalType> for Type
where
    Type: Into<InternalType>,
{
    fn into_internal(self) -> InternalType {
        self.into()
    }
}

#[uniffi::export]
pub fn new_non_fungible_global_id_from_string(
    string: String,
) -> Result<NonFungibleGlobalId> {
    InternalNonFungibleGlobalId::from_str(&string).map_result()
}

#[uniffi::export]
pub fn new_non_fungible_global_id_sample() -> NonFungibleGlobalId {
    InternalNonFungibleGlobalId::sample().into()
}

#[uniffi::export]
pub fn new_non_fungible_global_id_sample_other() -> NonFungibleGlobalId {
    InternalNonFungibleGlobalId::sample_other().into()
}

#[uniffi::export]
pub fn non_fungible_global_id_to_string(
    global_id: &NonFungibleGlobalId,
) -> String {
    global_id.into_internal().to_string()
}

#[uniffi::export]
pub fn non_fungible_global_id_formatted(
    global_id: &NonFungibleGlobalId,
    format: AddressFormat,
) -> String {
    global_id.into_internal().formatted(format)
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
