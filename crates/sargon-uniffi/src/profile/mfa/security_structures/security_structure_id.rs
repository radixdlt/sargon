use crate::prelude::*;

/// A stable and globally unique identifier of a `SecurityStructureOfFactorSources` the
/// user has created. Also used in `SecurityStructureOfFactorSourceIDs` and in
/// `SecurityStructureOfFactorInstances`.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Copy,
    derive_more::Display,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
#[serde(transparent)]
pub struct SecurityStructureID(pub(crate) Uuid);
uniffi::custom_newtype!(SecurityStructureID, Uuid);

