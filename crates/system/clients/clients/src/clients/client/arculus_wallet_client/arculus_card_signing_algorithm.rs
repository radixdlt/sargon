use crate::prelude::*;

/// The hash algorithms supported by the arculus card
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CardAlgorithm {
    Ecdsa = 1,
    Eddsa = 2,
    Schnorr = 3,
    Ristretto = 4,
    Cardano = 5,
}

impl CardAlgorithm {
    // Returns the raw value of the enum
    pub fn val(&self) -> u8 {
        *self as u8
    }
}
