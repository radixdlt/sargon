use crate::prelude::*;

/// The source of the public keys to derive.
#[derive(Clone, Debug, PartialEq)]
pub enum DerivePublicKeysSource {
    /// Derive the public keys from a known mnemonic.
    Mnemonic(MnemonicWithPassphrase),

    /// Derive the public keys from a factor source added to the current Profile.
    FactorSource(FactorSourceIDFromHash),
}
