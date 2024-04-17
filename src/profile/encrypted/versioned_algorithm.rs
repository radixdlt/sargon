use crate::prelude::*;

pub trait VersionOfAlgorithm {
    type Version;
    fn version(&self) -> Self::Version;
    fn description(&self) -> String;
}

pub trait VersionedAlgo: VersionOfAlgorithm + TryFrom<Self::Version> {}
