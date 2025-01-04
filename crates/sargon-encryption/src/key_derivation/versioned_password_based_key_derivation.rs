use crate::prelude::*;

pub trait VersionedPasswordBasedKeyDerivation: VersionOfAlgorithm {
    fn kdf(&self, password: impl AsRef<str>) -> Exactly32Bytes;
}
