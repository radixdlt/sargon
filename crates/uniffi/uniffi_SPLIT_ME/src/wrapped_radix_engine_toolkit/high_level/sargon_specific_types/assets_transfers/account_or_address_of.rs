use crate::prelude::*;
use sargon::TransferRecipient as InternalTransferRecipient;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
#[allow(clippy::large_enum_variant)] // we cannot Box<Account>, since Box is not UniFFI compatible.
pub enum TransferRecipient {
    ProfileAccount { value: AccountForDisplay },
    AddressOfExternalAccount { value: AccountAddress },
    RnsDomainConfiguredReceiver { value: RnsDomainConfiguredReceiver },
}

#[uniffi::export]
pub fn new_account_or_address_of_sample() -> TransferRecipient {
    InternalTransferRecipient::sample().into()
}

#[uniffi::export]
pub fn new_account_or_address_of_sample_other() -> TransferRecipient {
    InternalTransferRecipient::sample_other().into()
}

#[uniffi::export]
pub fn transfer_recipient_address(
    recipient: &TransferRecipient,
) -> AccountAddress {
    (*recipient.into_internal().account_address()).into()
}
