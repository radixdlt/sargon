use crate::{decl_secret_bytes, prelude::*};

decl_secret_bytes!(
    /// A BIP39 seed for hierarchal deterministic wallets, as per the [BIP39 standard][doc].
    ///
    /// We typically obtain this by calling [`to_seed` on `MnemonicWithPassphrase`][MnemonicWithPassphrase::to_seed].
    ///
    /// [doc]: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki#user-content-From_mnemonic_to_seed
    BIP39Seed,
    64
);
