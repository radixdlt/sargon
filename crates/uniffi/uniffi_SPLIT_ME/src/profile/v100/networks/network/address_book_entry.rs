use crate::prelude::*;
use sargon::AddressBookEntry as InternalAddressBookEntry;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AddressBookEntry {
    pub address: AccountAddress,
    pub name: DisplayName,
    pub note: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

delegate_debug_into!(AddressBookEntry, InternalAddressBookEntry);

#[uniffi::export]
pub fn new_address_book_entry_sample() -> AddressBookEntry {
    InternalAddressBookEntry::sample().into()
}

#[uniffi::export]
pub fn new_address_book_entry_sample_other() -> AddressBookEntry {
    InternalAddressBookEntry::sample_other().into()
}
