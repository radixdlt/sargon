use crate::prelude::*;
use sargon::FactorInstance as InternalFactorInstance;

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
pub struct FactorInstance {
    /// The ID of the `FactorSource` that was used to produce this
    /// factor instance. We will lookup the `FactorSource` in the
    /// `Profile` and can present user with instruction to re-access
    /// this factor source in order control the `badge`.
    pub factor_source_id: FactorSourceID,

    /// Either a "physical" badge (NFT) or some source for recreation of a producer
    /// of a virtual badge (signature), e.g. a HD derivation path, from which a private key
    /// is derived which produces virtual badges (signatures).
    pub badge: FactorInstanceBadge,
}

impl From<InternalFactorInstance> for FactorInstance {
    fn from(value: InternalFactorInstance) -> Self {
        Self {
            factor_source_id: value.factor_source_id.into(),
            badge: value.badge.into(),
        }
    }
}

impl Into<InternalFactorInstance> for FactorInstance {
    fn into(self) -> InternalFactorInstance {
        InternalFactorInstance {
            factor_source_id: self.factor_source_id.into(),
            badge: self.badge.into(),
        }
    }
}