use crate::prelude::*;

#[uniffi::export]
pub fn new_asset_exception_sample() -> AssetException {
    AssetException::sample()
}

#[uniffi::export]
pub fn new_asset_exception_sample_other() -> AssetException {
    AssetException::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AssetException;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_asset_exception_sample(),
                new_asset_exception_sample_other(),
                // duplicates should get removed
                new_asset_exception_sample(),
                new_asset_exception_sample_other(),
            ])
            .len(),
            2
        );
    }
}
