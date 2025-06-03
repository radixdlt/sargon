use crate::*;
use serde::{Deserialize, Serialize};

/// ProgrammaticScryptoSborValueKind : These are the Scrypto SBOR `ValueKind`s, but with `Bytes` added as an alias for `Vec`, to display such values as hex-encoded strings.
/// These are the Scrypto SBOR `ValueKind`s, but with `Bytes` added as an alias for `Vec`, to display such values as hex-encoded strings.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum ProgrammaticScryptoSborValueKind {
    Bool,

    I8,

    I16,

    I32,

    I64,

    I128,

    U8,

    U16,

    U32,

    U64,

    U128,

    String,

    Enum,

    Array,

    Bytes,

    Map,

    Tuple,

    Reference,

    Own,

    Decimal,

    PreciseDecimal,

    NonFungibleLocalId,
}

impl std::fmt::Display for ProgrammaticScryptoSborValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "Bool"),
            Self::I8 => write!(f, "I8"),
            Self::I16 => write!(f, "I16"),
            Self::I32 => write!(f, "I32"),
            Self::I64 => write!(f, "I64"),
            Self::I128 => write!(f, "I128"),
            Self::U8 => write!(f, "U8"),
            Self::U16 => write!(f, "U16"),
            Self::U32 => write!(f, "U32"),
            Self::U64 => write!(f, "U64"),
            Self::U128 => write!(f, "U128"),
            Self::String => write!(f, "String"),
            Self::Enum => write!(f, "Enum"),
            Self::Array => write!(f, "Array"),
            Self::Bytes => write!(f, "Bytes"),
            Self::Map => write!(f, "Map"),
            Self::Tuple => write!(f, "Tuple"),
            Self::Reference => write!(f, "Reference"),
            Self::Own => write!(f, "Own"),
            Self::Decimal => write!(f, "Decimal"),
            Self::PreciseDecimal => write!(f, "PreciseDecimal"),
            Self::NonFungibleLocalId => write!(f, "NonFungibleLocalId"),
        }
    }
}

impl Default for ProgrammaticScryptoSborValueKind {
    fn default() -> ProgrammaticScryptoSborValueKind {
        Self::Bool
    }
}
