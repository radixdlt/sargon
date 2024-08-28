use crate::prelude::*;

#[uniffi::export]
pub fn factor_sources_all_sample_values() -> Vec<FactorSource> {
    FactorSource::sample_values_all()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_of_samples() {
        assert!(factor_sources_all_sample_values().len() > 10);
    }
}
