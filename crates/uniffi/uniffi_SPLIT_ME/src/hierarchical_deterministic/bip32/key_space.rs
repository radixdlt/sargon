use crate::prelude::*;

use sargon::KeySpace as InternalKeySpace;

/// A discriminator of an `HDPathComponent`.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    InternalConversion,
    uniffi::Enum,
)]
pub enum KeySpace {
    Unsecurified { is_hardened: bool },
    Securified,
}

#[uniffi::export]
pub fn new_key_space_sample() -> KeySpace {
    InternalKeySpace::sample().into()
}

#[uniffi::export]
pub fn new_key_space_sample_other() -> KeySpace {
    InternalKeySpace::sample_other().into()
}
