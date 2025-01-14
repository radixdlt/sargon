use crate::prelude::*;

/// The curves supported by the arculus card
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum CardCurve {
    Secp256k1 = 0x0100,
    Ed25519 = 0x0201,
    Ed25519Blake2bNano = 0x0202,
    Ed25519Curve = 0x0203,
    Nist256p1 = 0x0301,
    Ed25519ExtendedCardano = 0x0401,
    Sr25519 = 0x0501,
}

impl CardCurve {
    pub fn val(&self) -> u16 {
        *self as u16
    }
}

impl From<SLIP10Curve> for CardCurve {
    fn from(value: SLIP10Curve) -> Self {
        match value {
            SLIP10Curve::Curve25519 => CardCurve::Ed25519Curve,
            SLIP10Curve::Secp256k1 => CardCurve::Secp256k1,
        }
    }
}
