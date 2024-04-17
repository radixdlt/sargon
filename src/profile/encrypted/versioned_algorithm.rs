use crate::prelude::*;

/// A versioned algorithm that can be initiated from some specific version.
pub trait VersionedAlgorithm:
    VersionOfAlgorithm + TryFrom<Self::Version>
{
}
