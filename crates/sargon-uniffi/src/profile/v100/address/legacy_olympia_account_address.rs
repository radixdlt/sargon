use crate::prelude::*;
use sargon::LegacyOlympiaAccountAddress as InternalLegacyOlympiaAccountAddress;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct LegacyOlympiaAccountAddress {
    value: Secp256k1PublicKey,
}

impl From<InternalLegacyOlympiaAccountAddress> for LegacyOlympiaAccountAddress {
    fn from(value: InternalLegacyOlympiaAccountAddress) -> Self {
        Self {
            value: value.public_key.into(),
        }
    }
}

impl Into<InternalLegacyOlympiaAccountAddress> for LegacyOlympiaAccountAddress {
    fn into(self) -> InternalLegacyOlympiaAccountAddress {
        InternalLegacyOlympiaAccountAddress {
            public_key: self.value.into(),
        }
    }
}

#[uniffi::export]
pub fn new_legacy_olympia_account_address_sample() -> LegacyOlympiaAccountAddress
{
    InternalLegacyOlympiaAccountAddress::sample().into()
}

#[uniffi::export]
pub fn new_legacy_olympia_account_address_sample_other(
) -> LegacyOlympiaAccountAddress {
    InternalLegacyOlympiaAccountAddress::sample_other().into()
}

#[uniffi::export]
pub fn new_legacy_olympia_account_address_from_public_key(
    public_key: Secp256k1PublicKey,
) -> LegacyOlympiaAccountAddress {
    InternalLegacyOlympiaAccountAddress::from(public_key.into_internal()).into()
}

#[uniffi::export]
pub fn new_legacy_olympia_account_address_from_string(
    string: String,
) -> Result<LegacyOlympiaAccountAddress> {
    InternalLegacyOlympiaAccountAddress::from_str(&string).map_result()
}

#[uniffi::export]
pub fn legacy_olympia_account_address_to_string(
    address: &LegacyOlympiaAccountAddress,
) -> String {
    address.into_internal().to_string()
}

#[uniffi::export]
pub fn legacy_olympia_account_address_formatted(
    address: &LegacyOlympiaAccountAddress,
    format: AddressFormat,
) -> String {
    address.into_internal().formatted(format.into_internal())
}

#[uniffi::export]
pub fn legacy_olympia_account_address_to_babylon_account_address(
    address: LegacyOlympiaAccountAddress,
) -> AccountAddress {
    address.into_internal().to_babylon_account_address().into()
}

#[uniffi::export]
pub fn legacy_olympia_account_address_is_legacy_of_babylon(
    legacy_olympia_address: &LegacyOlympiaAccountAddress,
    babylon_account_address: &AccountAddress,
) -> bool {
    babylon_account_address
        .into_internal()
        .was_migrated_from_legacy_olympia_account_address(
            &legacy_olympia_address.into_internal(),
        )
}
