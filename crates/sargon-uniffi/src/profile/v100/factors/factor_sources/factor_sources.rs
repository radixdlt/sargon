use crate::prelude::*;
use sargon::FactorSource as InternalFactorSource;

#[uniffi::export]
pub fn factor_sources_all_sample_values() -> Vec<FactorSource> {
    InternalFactorSource::sample_values_all().into_type()
}
