use crate::prelude::*;

#[uniffi::export]
pub fn new_identity_address_from(
    public_key: PublicKey,
    network_id: NetworkID,
) -> IdentityAddress {
    IdentityAddress::new(public_key, network_id)
}

#[uniffi::export]
pub fn new_identity_address_sample_mainnet() -> IdentityAddress {
    IdentityAddress::sample_mainnet()
}

#[uniffi::export]
pub fn new_identity_address_sample_mainnet_other() -> IdentityAddress {
    IdentityAddress::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_identity_address_sample_stokenet() -> IdentityAddress {
    IdentityAddress::sample_stokenet()
}

#[uniffi::export]
pub fn new_identity_address_sample_stokenet_other() -> IdentityAddress {
    IdentityAddress::sample_stokenet_other()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IdentityAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "identity_rdx122kttqch0eehzj6f9nkkxcw7msfeg9udurq5u0ysa0e92c59w0mg6x";
        let address = new_identity_address(b32.to_owned()).unwrap();
        assert_eq!(identity_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(identity_address_bech32_address(&address), b32);
    }

    #[test]
    fn new_from_public_key_bytes_and_network_id() {
        let public_key = Ed25519PublicKey::from_str(
            "6c28952be5cdade99c7dd5d003b6b692714b6b74c5fdb5fdc9a8e4ee1d297838",
        )
        .unwrap();
        assert_eq!(
            new_identity_address_from(PublicKey::Ed25519(public_key), NetworkID::Mainnet)
            .address(),
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j"
        )
    }

    #[test]
    fn sample() {
        assert_eq!(
            new_identity_address_sample_mainnet(),
            SUT::sample_mainnet()
        );

        assert_eq!(
            new_identity_address_sample_mainnet_other(),
            SUT::sample_mainnet_other()
        );

        assert_eq!(
            new_identity_address_sample_stokenet(),
            SUT::sample_stokenet()
        );

        assert_eq!(
            new_identity_address_sample_stokenet_other(),
            SUT::sample_stokenet_other()
        );
    }
}
