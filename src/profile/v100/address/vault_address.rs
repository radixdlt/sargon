use crate::prelude::*;

use radix_engine_toolkit::models::canonical_address_types::CanonicalVaultAddress as RetVaultAddress;

/// Addresses to a specific vault, owned by a user, holding asset of one kind, either fungible or non-fungible.
/// Identities cannot own assets so they do not have vaults, but Accounts do, e.g.:
/// `"internal_vault_rdx1tz474x29nxxd4k2p2reete9xyz4apawv63dphxkr00qt23vyju49fq"`
///
/// There are fundamentally two different sub-types ([Scrypto's `EntityType`][entt]) of VaultAddresses:
/// * InternalFungibleVault
/// * InternalNonFungibleVault
///
/// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalVaultAddress`][ret], and
/// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
///
/// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
/// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L251-L255
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::FromStr,
    derive_more::Display,
    SerializeDisplay,
    DeserializeFromStr,
    uniffi::Record,
)]
#[display("{secret_magic}")]
pub struct VaultAddress {
    /// @Kotlin / Swift developer: Do NOT use this property/field. Instead use all the provided methods on this address type.
    /// (which are in fact vendored as freestanding global functions,
    /// due to limitations in UniFII as of Feb 2024, but you should
    /// create extension methods on this address type in FFI land, translating
    /// these functions into methods.)
    pub(crate) secret_magic: RetVaultAddress,
}

impl VaultAddress {
    pub fn is_fungible(&self) -> bool {
        self.secret_magic.is_fungible()
    }

    pub fn is_non_fungible(&self) -> bool {
        self.secret_magic.is_non_fungible()
    }
}

#[uniffi::export]
pub fn vault_address_is_fungible(address: &VaultAddress) -> bool {
    address.is_fungible()
}

#[uniffi::export]
pub fn vault_address_is_non_fungible(address: &VaultAddress) -> bool {
    address.is_non_fungible()
}

#[uniffi::export]
pub fn new_vault_address_placeholder_mainnet_fungible() -> VaultAddress {
    VaultAddress::placeholder_mainnet_fungible()
}

#[uniffi::export]
pub fn new_vault_address_placeholder_mainnet_non_fungible() -> VaultAddress {
    VaultAddress::placeholder_mainnet_non_fungible()
}

#[uniffi::export]
pub fn new_vault_address_placeholder_stokenet_fungible() -> VaultAddress {
    VaultAddress::placeholder_stokenet_fungible()
}

#[uniffi::export]
pub fn new_vault_address_placeholder_stokenet_non_fungible() -> VaultAddress {
    VaultAddress::placeholder_stokenet_non_fungible()
}

impl VaultAddress {
    pub fn placeholder_mainnet_fungible() -> Self {
        "internal_vault_rdx1tz474x29nxxd4k2p2reete9xyz4apawv63dphxkr00qt23vyju49fq".parse().expect("Placeholder")
    }

    pub fn placeholder_mainnet_non_fungible() -> Self {
        "internal_vault_rdx1nz8nsf78zrrxq7r2r059vzsnmleawr36470chegf84p06fhze6wq3d".parse().expect("Placeholder")
    }

    pub fn placeholder_stokenet_fungible() -> Self {
        "internal_vault_tdx_2_1tqulaapn7etkm8d7h7h2dl5wn32dhmgj942mjc8g4jm9qajga6e40s".parse().expect("Placeholder")
    }

    pub fn placeholder_stokenet_non_fungible() -> Self {
        "internal_vault_tdx_2_1npcfs62psk2r8fnchjfrc5fepk7defxzl5c4ngsun9h0svf5zts4te".parse().expect("Placeholder")
    }
}

impl HasPlaceholder for VaultAddress {
    fn placeholder() -> Self {
        Self::placeholder_mainnet_fungible()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_mainnet_non_fungible()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = VaultAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
    }

    #[test]
    fn is_fungible() {
        assert_eq!(SUT::placeholder_mainnet_fungible().is_fungible(), true);
        assert_eq!(
            SUT::placeholder_mainnet_fungible().is_non_fungible(),
            false
        );

        assert_eq!(SUT::placeholder_stokenet_fungible().is_fungible(), true);
        assert_eq!(
            SUT::placeholder_stokenet_fungible().is_non_fungible(),
            false
        );
    }

    #[test]
    fn display() {
        let s = "internal_vault_rdx1nz8nsf78zrrxq7r2r059vzsnmleawr36470chegf84p06fhze6wq3d";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{a}"), s);
    }

    #[test]
    fn json_roundtrip() {
        let a: SUT =
            "internal_vault_rdx1nz8nsf78zrrxq7r2r059vzsnmleawr36470chegf84p06fhze6wq3d"
                .parse()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &a,
            json!("internal_vault_rdx1nz8nsf78zrrxq7r2r059vzsnmleawr36470chegf84p06fhze6wq3d"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("internal_vault_rdx1tz474x29nxxd4k2p2reete9xyz4apawv63dphxkr00qt23vyju49fq"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(
            json!("internal_vault_rdx1nz8nsf78zrrxq7r2r059vzsnmleawr36470chegf84p06fhze6wq3e")
        );
        assert_json_value_fails::<SUT>(
            json!("account_rdx1nz8nsf78zrrxq7r2r059vzsnmleawr36470chegf84p06fhze6wq3d")
        );
        assert_json_value_fails::<SUT>(json!("super invalid"));
    }

    #[test]
    fn network_id_stokenet() {
        let a: SUT =
            "internal_vault_tdx_2_1tqulaapn7etkm8d7h7h2dl5wn32dhmgj942mjc8g4jm9qajga6e40s"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Stokenet);
    }

    #[test]
    fn network_id_mainnet() {
        let a: SUT =
            "internal_vault_rdx1nz8nsf78zrrxq7r2r059vzsnmleawr36470chegf84p06fhze6wq3d"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Mainnet);
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

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
        assert_eq!(
            vault_address_is_fungible(&SUT::placeholder_mainnet_fungible()),
            true
        );
        assert_eq!(
            vault_address_is_non_fungible(&SUT::placeholder_mainnet_fungible()),
            false
        );

        assert_eq!(
            vault_address_is_fungible(&SUT::placeholder_stokenet_fungible()),
            true
        );
        assert_eq!(
            vault_address_is_non_fungible(&SUT::placeholder_stokenet_fungible()),
            false
        );
    }

    #[test]
    fn placeholder() {
        assert_eq!(
            new_vault_address_placeholder_mainnet_fungible(),
            SUT::placeholder_mainnet_fungible()
        );

        assert_eq!(
            new_vault_address_placeholder_mainnet_non_fungible(),
            SUT::placeholder_mainnet_non_fungible()
        );

        assert_eq!(
            new_vault_address_placeholder_stokenet_fungible(),
            SUT::placeholder_stokenet_fungible()
        );

        assert_eq!(
            new_vault_address_placeholder_stokenet_non_fungible(),
            SUT::placeholder_stokenet_non_fungible()
        );
    }
}
