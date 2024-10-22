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
    derive_more::FromStr,
    DeserializeFromStr,
    derive_more::Display,
)]
pub struct HDIndexSecurified(U30);
