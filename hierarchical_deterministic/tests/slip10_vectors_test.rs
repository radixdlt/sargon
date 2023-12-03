use hierarchical_deterministic::{
    bip39::mnemonic::Mnemonic, bip44::bip44_like_path::BIP44LikePath,
    derivation::mnemonic_with_passphrase::MnemonicWithPassphrase,
};

#[test]
fn derive_a_secp256k1_key_with_bip44_olympia() {
    let mwp = MnemonicWithPassphrase::with_passphrase(
            Mnemonic::from_phrase(
     "habit special recipe upon giraffe manual evil badge dwarf welcome inspire shrug post arrive van",
            )
            .unwrap(),
            "".to_string(),
        );

    let private_key =
        mwp.derive_private_key(BIP44LikePath::from_str("m/44H/1022H/0H/0/5H").unwrap());

    assert_eq!(
        "111323d507d9d690836798e3ef2e5292cfd31092b75b9b59fa584ff593a3d7e4",
        private_key.to_hex()
    );

    assert_eq!(
        "03e78cdb2e0b7ea6e55e121a58560ccf841a913d3a4a9b8349e0ef00c2102f48d8",
        private_key.public_key().to_hex()
    );
}
