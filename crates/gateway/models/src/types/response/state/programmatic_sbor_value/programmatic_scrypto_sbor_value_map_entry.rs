

use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProgrammaticScryptoSborValueMapEntry {

    pub key: Box<ProgrammaticScryptoSborValue>,

    pub value: Box<ProgrammaticScryptoSborValue>,
}

impl ProgrammaticScryptoSborValueMapEntry {
    pub fn new(key: ProgrammaticScryptoSborValue, value: ProgrammaticScryptoSborValue) -> ProgrammaticScryptoSborValueMapEntry {
        ProgrammaticScryptoSborValueMapEntry {
            key: Box::new(key),
            value: Box::new(value),
        }
    }
}

