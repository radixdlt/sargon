use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
)]
pub enum PasswordBasedKeyDerivationScheme {
    Version1,
}
