// use crate::prelude::*;
// use sargon::TrustedContactFactorSource as InternalTrustedContactFactorSource;
//
// /// A factor source representing a person, company, organization or otherwise
// /// entity that the user trusts to help her with recovery, if ever needed.
// #[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
// pub struct TrustedContactFactorSource {
//     /// Unique and stable identifier of this factor source.
//     pub id: FactorSourceIDFromAddress,
//
//     /// Common properties shared between FactorSources of different kinds,
//     /// describing its state, when added, and supported cryptographic parameters.
//     pub common: FactorSourceCommon,
//
//     /// The contact information about the contact that is 'trusted'.
//     pub contact: TrustedContactFactorSourceContact,
// }
//
// #[uniffi::export]
// pub fn new_trusted_contact_factor_source_sample() -> TrustedContactFactorSource
// {
//     InternalTrustedContactFactorSource::sample().into()
// }
//
// #[uniffi::export]
// pub fn new_trusted_contact_factor_source_sample_other(
// ) -> TrustedContactFactorSource {
//     InternalTrustedContactFactorSource::sample_other().into()
// }
//
// #[uniffi::export]
// fn new_trusted_contact_factor_source_from_address_and_contact(
//     account_address: AccountAddress,
//     contact: TrustedContactFactorSourceContact,
// ) -> TrustedContactFactorSource {
//     InternalTrustedContactFactorSource::new(
//         account_address.to_string(),
//         contact.into_internal(),
//     )
//     .into()
// }
