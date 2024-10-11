use crate::prelude::*;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    enum_iterator::Sequence,
)]
pub enum AddressFormat {
    Full,
    Raw,
    Default,
}

impl HasSampleValues for AddressFormat {
    fn sample() -> Self {
        AddressFormat::Full
    }

    fn sample_other() -> Self {
        AddressFormat::Raw
    }
}