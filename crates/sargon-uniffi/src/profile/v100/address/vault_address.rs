use crate::prelude::*;

decl_ret_wrapped_address!(
    /// Addresses to a specific vault, owned by a user, holding asset of one kind, either fungible or non-fungible.
    /// Identities cannot own assets so they do not have vaults, but Accounts do, e.g.:
    /// `"internal_vault_rdx1tz474x29nxxd4k2p2reete9xyz4apawv63dphxkr00qt23vyju49fq"`
    ///
    /// There are fundamentally two different sub-types ([Scrypto's `EntityType`][entt]) of VaultAddresses:
    /// * InternalFungibleVault
    /// * InternalNonFungibleVault
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalVaultAddress`][ret], and
    /// give it UniFFI support, as a ` uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L251-L255
    vault
);

#[uniffi::export]
pub fn vault_address_is_fungible(address: &VaultAddress) -> bool {
    address.into_internal().is_fungible()
}

#[uniffi::export]
pub fn vault_address_is_non_fungible(address: &VaultAddress) -> bool {
    address.into_internal().is_non_fungible()
}

#[uniffi::export]
pub fn new_vault_address_sample_mainnet_fungible() -> VaultAddress {
    InternalAddress::sample_mainnet().into()
}

#[uniffi::export]
pub fn new_vault_address_sample_mainnet_non_fungible() -> VaultAddress {
    InternalAddress::sample_mainnet_other().into()
}

#[uniffi::export]
pub fn new_vault_address_sample_stokenet_fungible() -> VaultAddress {
    InternalAddress::sample_stokenet().into()
}

#[uniffi::export]
pub fn new_vault_address_sample_stokenet_non_fungible() -> VaultAddress {
    InternalAddress::sample_stokenet_other().into()
}

#[cfg(test)]
mod uniffi_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = VaultAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "internal_vault_rdx1nz8nsf78zrrxq7r2r059vzsnmleawr36470chegf84p06fhze6wq3d";
        let address = new_vault_address(b32.to_owned()).unwrap();
        assert_eq!(vault_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(vault_address_bech32_address(&address), b32);
    }

    #[test]
    fn new() {
        let s = "internal_vault_rdx1tz474x29nxxd4k2p2reete9xyz4apawv63dphxkr00qt23vyju49fq";
        let a = SUT::try_from_bech32(s).unwrap();
        let b = new_vault_address(s.to_string()).unwrap();
        assert_eq!(b.address(), s);
        assert_eq!(a, b);
    }

    #[test]
    fn is_fungible() {
        assert!(vault_address_is_fungible(&SUT::sample_mainnet_fungible()));
        assert!(!vault_address_is_non_fungible(
            &SUT::sample_mainnet_fungible()
        ));

        assert!(vault_address_is_fungible(&SUT::sample_stokenet_fungible()));
        assert!(!vault_address_is_non_fungible(
            &SUT::sample_stokenet_fungible()
        ));
    }

    #[test]
    fn sample() {
        assert_eq!(
            new_vault_address_sample_mainnet_fungible(),
            SUT::sample_mainnet_fungible()
        );

        assert_eq!(
            new_vault_address_sample_mainnet_non_fungible(),
            SUT::sample_mainnet_non_fungible()
        );

        assert_eq!(
            new_vault_address_sample_stokenet_fungible(),
            SUT::sample_stokenet_fungible()
        );

        assert_eq!(
            new_vault_address_sample_stokenet_non_fungible(),
            SUT::sample_stokenet_non_fungible()
        );
    }
}
