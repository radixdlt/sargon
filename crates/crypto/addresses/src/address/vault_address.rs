use crate::prelude::*;

decl_address!(
    /// Addresses to a specific vault, owned by a user, holding asset of one kind, either fungible or non_fungible.
    /// Identities cannot own assets so they do not have vaults, but Accounts do, e.g.:
    /// `"internal_vault_rdx1tz474x29nxxd4k2p2reete9xyz4apawv63dphxkr00qt23vyju49fq"`
    ///
    /// There are fundamentally two different sub-types ([Scrypto's `EntityType`][entt]) of VaultAddresses:
    /// * InternalFungibleVault
    /// * InternalNonFungibleVault
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalVaultAddress`][ret].
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L251-L255
    vault => [
        ScryptoEntityType::InternalFungibleVault,
        ScryptoEntityType::InternalNonFungibleVault
    ]
);

impl VaultAddress {
    pub fn is_fungible(&self) -> bool {
        self.node_id.is_internal_fungible_vault()
    }

    pub fn is_non_fungible(&self) -> bool {
        !self.is_fungible()
    }
}

impl VaultAddress {
    pub fn sample_mainnet() -> Self {
        Self::sample_mainnet_fungible()
    }

    pub fn sample_mainnet_other() -> Self {
        Self::sample_mainnet_non_fungible()
    }

    pub fn sample_stokenet() -> Self {
        Self::sample_stokenet_fungible()
    }

    pub fn sample_stokenet_other() -> Self {
        Self::sample_stokenet_non_fungible()
    }
}

impl VaultAddress {
    pub fn sample_mainnet_fungible() -> Self {
        "internal_vault_rdx1tz474x29nxxd4k2p2reete9xyz4apawv63dphxkr00qt23vyju49fq".parse().expect("Sample")
    }

    pub fn sample_mainnet_non_fungible() -> Self {
        "internal_vault_rdx1nz8nsf78zrrxq7r2r059vzsnmleawr36470chegf84p06fhze6wq3d".parse().expect("Sample")
    }

    pub fn sample_stokenet_fungible() -> Self {
        "internal_vault_tdx_2_1tqulaapn7etkm8d7h7h2dl5wn32dhmgj942mjc8g4jm9qajga6e40s".parse().expect("Sample")
    }

    pub fn sample_stokenet_non_fungible() -> Self {
        "internal_vault_tdx_2_1npcfs62psk2r8fnchjfrc5fepk7defxzl5c4ngsun9h0svf5zts4te".parse().expect("Sample")
    }
}

impl HasSampleValues for VaultAddress {
    fn sample() -> Self {
        Self::sample_mainnet_fungible()
    }

    fn sample_other() -> Self {
        Self::sample_mainnet_non_fungible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = VaultAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn is_fungible() {
        assert!(SUT::sample_mainnet_fungible().is_fungible());
        assert!(!SUT::sample_mainnet_fungible().is_non_fungible());

        assert!(SUT::sample_stokenet_fungible().is_fungible());
        assert!(!SUT::sample_stokenet_fungible().is_non_fungible());
    }

    #[test]
    fn display() {
        let s = "internal_vault_rdx1nz8nsf78zrrxq7r2r059vzsnmleawr36470chegf84p06fhze6wq3d";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{}", a), s);
    }

    #[test]
    fn debug() {
        let s = "internal_vault_rdx1nz8nsf78zrrxq7r2r059vzsnmleawr36470chegf84p06fhze6wq3d";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{:?}", a), s);
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
