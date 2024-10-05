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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BIP39Seed;

    #[test]
    fn zeroize() {
        let mut sut: SUT = MnemonicWithPassphrase::sample().to_seed();
        assert!(!sut.is_zeroized());
        sut.zeroize();
        assert!(sut.is_zeroized());
    }

    #[test]
    fn manual_uniffi_conversion() {
        let bytes = Exactly64Bytes::sample();
        let builtin: BagOfBytes = bytes.clone().as_ref().into();
        let sut = new_b_i_p39_seed_from_bytes(builtin.clone()).unwrap();
        let rust_side = sut.secret_magic;

        let ffi_side =
        <BIP39SeedSecretMagic as crate::UniffiCustomTypeConverter>::from_custom(
            rust_side,
        );

        assert_eq!(ffi_side.to_hex(), builtin.to_hex());

        let from_ffi_side =
        <BIP39SeedSecretMagic as crate::UniffiCustomTypeConverter>::into_custom(
            ffi_side,
        )
        .unwrap();

        assert_eq!(
            new_b_i_p39_seed_from_bytes(builtin.clone())
                .unwrap()
                .secret_magic
                .0,
            from_ffi_side.0
        );
    }
}
