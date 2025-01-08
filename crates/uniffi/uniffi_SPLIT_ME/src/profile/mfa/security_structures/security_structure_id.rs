use crate::prelude::*;
use sargon::SecurityStructureID as InternalSecurityStructureID;

uniffi::custom_newtype!(SecurityStructureID, Uuid);

/// A stable and globally unique identifier of a `SecurityStructureOfFactorSources` the
/// user has created. Also used in `SecurityStructureOfFactorSourceIDs` and in
/// `SecurityStructureOfFactorInstances`.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion)]
pub struct SecurityStructureID(pub Uuid);
