use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

uniffi::custom_newtype!(WalletInteractionVersion, u64);

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct WalletInteractionVersion(pub u64);

impl From<u64> for WalletInteractionVersion {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl WalletInteractionVersion {
    pub fn current() -> Self {
        Self(1)
    }
}

impl HasSampleValues for WalletInteractionVersion {
    fn sample() -> Self {
        Self(1)
    }

    fn sample_other() -> Self {
        Self(2)
    }
}
