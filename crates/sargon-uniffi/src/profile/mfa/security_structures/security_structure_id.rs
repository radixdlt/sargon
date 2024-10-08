use crate::prelude::*;
use sargon::SecurityStructureID as InternalSecurityStructureID;

/// A stable and globally unique identifier of a `SecurityStructureOfFactorSources` the
/// user has created. Also used in `SecurityStructureOfFactorSourceIDs` and in
/// `SecurityStructureOfFactorInstances`.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct SecurityStructureID {
    pub value: Uuid,
}

impl From<InternalSecurityStructureID> for SecurityStructureID {
    fn from(value: InternalSecurityStructureID) -> Self {
        SecurityStructureID {
            value: value.0,
        }
    }
}

impl Into<InternalSecurityStructureID> for SecurityStructureID {
    fn into(self) -> InternalSecurityStructureID {
        InternalSecurityStructureID(self.value)
    }
}

