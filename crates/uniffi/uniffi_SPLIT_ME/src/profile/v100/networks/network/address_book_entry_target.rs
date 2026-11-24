use crate::prelude::*;
use sargon::AddressBookEntryTarget as InternalAddressBookEntryTarget;

/// What an [`AddressBookEntry`] names: an address, e.g. of an account, or a
/// non-fungible, e.g. the signature resource of someone's key.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum AddressBookEntryTarget {
    GlobalAddress { value: Address },

    NonFungible { value: NonFungibleGlobalId },
}

#[uniffi::export]
pub fn new_address_book_entry_target_sample() -> AddressBookEntryTarget {
    InternalAddressBookEntryTarget::sample().into()
}

#[uniffi::export]
pub fn new_address_book_entry_target_sample_other() -> AddressBookEntryTarget {
    InternalAddressBookEntryTarget::sample_other().into()
}

decl_conversion_tests_for!(AddressBookEntryTarget);
