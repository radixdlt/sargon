use crate::prelude::*;
use sargon::OwnedOrThirdPartyAccountAddress as InternalAccountOrAddressOf;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
#[allow(clippy::large_enum_variant)] // we cannot Box<Account>, since Box is not UniFFI compatible.
pub enum OwnedOrThirdPartyAccountAddress {
    OwnedAccount { value: Account },
    ThirdPartyAccount { value: AccountAddress },
}

#[uniffi::export]
pub fn new_account_or_address_of_sample() -> OwnedOrThirdPartyAccountAddress {
    InternalAccountOrAddressOf::sample().into()
}

#[uniffi::export]
pub fn new_account_or_address_of_sample_other(
) -> OwnedOrThirdPartyAccountAddress {
    InternalAccountOrAddressOf::sample_other().into()
}

#[uniffi::export]
pub fn account_or_address_of_account_address(
    recipient: &OwnedOrThirdPartyAccountAddress,
) -> AccountAddress {
    (*recipient.into_internal().account_address()).into()
}
