use crate::prelude::*;

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    derive_more::Display,
    Ord,
    derive_more::FromStr,
    Hash,
)]
pub struct HDIndexUnhardened(U30);

impl HDIndexUnhardened {
    pub const fn new(inner: U30) -> Self {
        Self(inner)
    }
}

impl Deref for HDIndexUnhardened {
    type Target = U30;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
