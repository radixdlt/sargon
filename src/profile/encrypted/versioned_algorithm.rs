use crate::prelude::*;

pub trait VersionedAlgorithm: From<Self::Version> {
    type Version;
    fn version(&self) -> Self::Version;
    fn description(&self) -> String;
}
