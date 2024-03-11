use crate::prelude::*;

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
pub fn new_vault_address_sample_mainnet_fungible() -> VaultAddress {
    VaultAddress::sample_mainnet_fungible()
}

#[uniffi::export]
pub fn new_vault_address_sample_mainnet_non_fungible() -> VaultAddress {
    VaultAddress::sample_mainnet_non_fungible()
}

#[uniffi::export]
pub fn new_vault_address_sample_stokenet_fungible() -> VaultAddress {
    VaultAddress::sample_stokenet_fungible()
}

#[uniffi::export]
pub fn new_vault_address_sample_stokenet_non_fungible() -> VaultAddress {
    VaultAddress::sample_stokenet_non_fungible()
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
    use crate::prelude::*;

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
    fn manual_perform_uniffi_conversion() {
        type RetAddr = <SUT as FromRetAddress>::RetAddress;
        let sut = SUT::sample();
        let bech32 = sut.to_string();
        let ret = RetAddr::try_from_bech32(&bech32).unwrap();

        let ffi_side =
            <RetAddr as crate::UniffiCustomTypeConverter>::from_custom(ret);
        assert_eq!(ffi_side, bech32);
        let from_ffi_side =
            <RetAddr as crate::UniffiCustomTypeConverter>::into_custom(
                ffi_side,
            )
            .unwrap();
        assert_eq!(ret, from_ffi_side);
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
