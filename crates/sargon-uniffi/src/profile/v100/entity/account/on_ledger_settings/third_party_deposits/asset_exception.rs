use crate::prelude::*;
use sargon::AssetException as InternalAssetException;

decl_vec_samples_for!(AssetsExceptionList, AssetException);

/// The specific Asset exception rule, which overrides the general
///  `deposit_rule` of a `ThirdPartyDeposits` settings.
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct AssetException {
    /// Address of an asset to either deny or allow, as an exception overriding the `ThirdPartyDeposits`'s general `deposit_rule`.
    pub address: ResourceAddress,

    /// Either deny or allow the `address`.
    pub exception_rule: DepositAddressExceptionRule,
}

impl AssetException {
    pub fn into_internal(&self) -> InternalAssetException {
        self.clone().into()
    }
}

impl From<InternalAssetException> for AssetException {
    fn from(value: InternalAssetException) -> Self {
        Self {
            address: value.address.into(),
            exception_rule: value.exception_rule.into(),
        }
    }
}

impl Into<InternalAssetException> for AssetException {
    fn into(self) -> InternalAssetException {
        InternalAssetException {
            address: self.address.into(),
            exception_rule: self.exception_rule.into(),
        }
    }
}

#[uniffi::export]
pub fn new_asset_exception_sample() -> AssetException {
    InternalAssetException::sample().into()
}

#[uniffi::export]
pub fn new_asset_exception_sample_other() -> AssetException {
    InternalAssetException::sample_other().into()
}

decl_conversion_tests_for!(AssetException);
