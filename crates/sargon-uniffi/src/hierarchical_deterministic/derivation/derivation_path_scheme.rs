use crate::prelude::*;

/// Which derivation path to used for some particular HD operations
/// such as signing or public key derivation. Radix Babylon introduces
/// a new scheme call Cap26 but we also need to support BIP44-like used
/// by Olympia.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    uniffi::Enum,
)]
pub enum DerivationPathScheme {
    /// A BIP32 based derivation path scheme, using SLIP10.
    #[serde(rename = "cap26")]
    Cap26,

    /// A BIP32 based similar to BIP44, but not strict BIP44 since the
    /// last path component is hardened (a mistake made during Olympia),
    /// used to support legacy accounts imported from Olympia wallet.
    #[serde(rename = "bip44Olympia")]
    Bip44Olympia,
}

impl Identifiable for DerivationPathScheme {
    type ID = String;

    fn id(&self) -> Self::ID {
        match self {
            Self::Cap26 => "cap26".to_string(),
            Self::Bip44Olympia => "bip44Olympia".to_string(),
        }
    }
}