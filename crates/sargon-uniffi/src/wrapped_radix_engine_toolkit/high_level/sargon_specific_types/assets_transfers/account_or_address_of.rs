use crate::prelude::*;
use sargon::AccountOrAddressOf as InternalAccountOrAddressOf;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
#[allow(clippy::large_enum_variant)] // we cannot Box<Account>, since Box is not UniFFI compatible.
pub enum AccountOrAddressOf {
    ProfileAccount { value: Account },
    AddressOfExternalAccount { value: AccountAddress },
}

#[uniffi::export]
pub fn new_account_or_address_of_sample() -> AccountOrAddressOf {
    InternalAccountOrAddressOf::sample().into()
}

#[uniffi::export]
pub fn new_account_or_address_of_sample_other() -> AccountOrAddressOf {
    InternalAccountOrAddressOf::sample_other().into()
}

#[uniffi::export]
pub fn account_or_address_of_account_address(
    recipient: &AccountOrAddressOf,
) -> AccountAddress {
    recipient.into_internal().account_address().clone().into()
}
