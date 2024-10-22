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
    SerializeDisplay,
    derive_more::Display,
)]
pub enum HDIndexUnsecurified {
    Unhardened(HDIndexUnhardened),
    Hardened(HDIndexHardenedUnsecurified),
}
