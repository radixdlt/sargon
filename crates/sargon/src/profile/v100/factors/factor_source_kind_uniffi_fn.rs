use crate::prelude::*;

#[uniffi::export]
pub fn new_factor_source_kind_sample() -> FactorSourceKind {
    FactorSourceKind::sample()
}

#[uniffi::export]
pub fn new_factor_source_kind_sample_other() -> FactorSourceKind {
    FactorSourceKind::sample_other()
}

#[uniffi::export]
pub fn new_factor_source_kind_from_string(
    string: String,
) -> Result<FactorSourceKind> {
    FactorSourceKind::from_str(&string)
}

#[uniffi::export]
pub fn factor_source_kind_to_string(kind: FactorSourceKind) -> String {
    kind.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSourceKind;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_factor_source_kind_sample(),
                new_factor_source_kind_sample_other(),
                // duplicates should get removed
                new_factor_source_kind_sample(),
                new_factor_source_kind_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn string_roundtrip() {
        let sut = SUT::sample();
        let str = factor_source_kind_to_string(sut);
        let from_str = new_factor_source_kind_from_string(str).unwrap();
        assert_eq!(sut, from_str);
    }
}
