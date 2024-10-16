use crate::prelude::*;
use sargon::FactorSource as InternalFactorSource;

decl_identified_vec_of!(
    /// A collection of [`FactorSource`]s generated by a wallet or manually added by user.
    /// MUST never be empty.
    FactorSource
);

#[uniffi::export]
pub fn factor_sources_all_sample_values() -> Vec<FactorSource> {
    InternalFactorSource::sample_values_all().into_type()
}
