use crate::prelude::*;

/// The specific Asset exception rule, which overrides the general
///  `deposit_rule` of a `ThirdPartyDeposits` settings.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[display("rule: {}, address: {}", self.exception_rule, self.address)]
pub struct AssetException {
    /// Address of an asset to either deny or allow, as an exception overriding the `ThirdPartyDeposits`'s general `deposit_rule`.
    pub address: ResourceAddress,

    /// Either deny or allow the `address`.
    pub exception_rule: DepositAddressExceptionRule,
}

impl Identifiable for AssetException {
    type ID = ResourceAddress;

    fn id(&self) -> Self::ID {
        self.address
    }
}

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
