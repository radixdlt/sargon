use crate::prelude::*;
use sargon::FactorSourceIDFromAddress as InternalFactorSourceIDFromAddress;

/// FactorSourceID from an AccountAddress, typically used by `trustedContact` FactorSource.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct FactorSourceIDFromAddress {
    /// The kind of the FactorSource this ID refers to, typically `trustedContact`.
    pub kind: FactorSourceKind,

    /// An account address which the FactorSource this ID refers uses/needs.
    pub body: AccountAddress,
}

json_data_convertible!(FactorSourceIDFromAddress);

#[uniffi::export]
pub fn factor_source_id_from_address_to_string(
    factor_source_id: &FactorSourceIDFromAddress,
) -> String {
    factor_source_id.into_internal().to_string()
}

#[uniffi::export]
pub fn new_factor_source_id_from_address_sample() -> FactorSourceIDFromAddress {
    InternalFactorSourceIDFromAddress::sample().into()
}

#[uniffi::export]
pub fn new_factor_source_id_from_address_sample_other(
) -> FactorSourceIDFromAddress {
    InternalFactorSourceIDFromAddress::sample_other().into()
}
