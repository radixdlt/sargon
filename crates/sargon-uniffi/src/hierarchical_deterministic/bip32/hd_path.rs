use crate::prelude::*;

#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[display("{}", self.bip32_string())]
#[debug("{}", self.bip32_string())]
pub struct HDPath {
    pub components: Vec<HDPathComponent>,
}