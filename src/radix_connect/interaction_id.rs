use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

uniffi::custom_newtype!(WalletInteractionId, String);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WalletInteractionId(pub String);

impl From<String> for WalletInteractionId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for WalletInteractionId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl HasSampleValues for WalletInteractionId {
    fn sample() -> Self {
        Self("sample".to_string())
    }

    fn sample_other() -> Self {
        Self("sample_other".to_string())
    }
}
