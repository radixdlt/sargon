use crate::prelude::*;

use radix_engine_toolkit::models::canonical_address_types::CanonicalValidatorAddress as RetValidatorAddress;

#[uniffi::export]
pub fn new_validator_address_placeholder_mainnet() -> ValidatorAddress {
    ValidatorAddress::placeholder_mainnet()
}

#[uniffi::export]
pub fn new_validator_address_placeholder_mainnet_other() -> ValidatorAddress {
    ValidatorAddress::placeholder_mainnet_other()
}

#[uniffi::export]
pub fn new_validator_address_placeholder_stokenet() -> ValidatorAddress {
    ValidatorAddress::placeholder_stokenet()
}

#[uniffi::export]
pub fn new_validator_address_placeholder_stokenet_other() -> ValidatorAddress {
    ValidatorAddress::placeholder_stokenet_other()
}

impl HasPlaceholder for ValidatorAddress {
    fn placeholder() -> Self {
        Self::placeholder_mainnet()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_mainnet_other()
    }
}

impl ValidatorAddress {
    pub fn placeholder_mainnet() -> Self {
        "validator_rdx1sd5368vqdmjk0y2w7ymdts02cz9c52858gpyny56xdvzuheepdeyy0"
            .parse()
            .expect("Valid placeholder")
    }

    pub fn placeholder_mainnet_other() -> Self {
        "validator_rdx1sw5rrhkxs65kl9xcxu7t9yu3k8ptscjwamum4phclk297j6r28g8kd"
            .parse()
            .expect("Valid placeholder other")
    }

    pub fn placeholder_stokenet() -> Self {
        "validator_tdx_2_1sdatqsl6rx05yy2yvpf6ckfl7x8dluvzkcyljkn0x4lxkgucc0xz2w".parse().expect("Valid placeholder")
    }

    pub fn placeholder_stokenet_other() -> Self {
        "validator_tdx_2_1sdtnujyn3720ymg8lakydkvc5tw4q3zecdj95akdwt9de362mvtd94".parse().expect("Valid placeholder")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ValidatorAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());

        assert_eq!(SUT::placeholder_stokenet(), SUT::placeholder_stokenet());
        assert_eq!(
            SUT::placeholder_stokenet_other(),
            SUT::placeholder_stokenet_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
        assert_ne!(SUT::placeholder(), SUT::placeholder_stokenet());
    }

    #[test]
    fn display() {
        let s = "validator_rdx1sdcmd3ymwzvswgyva8lpknqrzuzzmmkac9my4auk29j5feumfh77fs";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{a}"), s);
    }

    #[test]
    fn json_roundtrip() {
        let a: SUT =
            "validator_rdx1sd4eq4vvnrmtxy0l4wxaykugwjmyflnnkn4sz3p9jv79ac2sv5sh88"
                .parse()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &a,
            json!("validator_rdx1sd4eq4vvnrmtxy0l4wxaykugwjmyflnnkn4sz3p9jv79ac2sv5sh88"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("validator_rdx1sdcmd3ymwzvswgyva8lpknqrzuzzmmkac9my4auk29j5feumfh77fs"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(
            json!("validator_rdx1sdcmd3ymwzvswgyva8lpknqrzuzzmmkac9my4auk29j5feumfh77ff")
        );
        assert_json_value_fails::<SUT>(
            json!("account_rdx1sdcmd3ymwzvswgyva8lpknqrzuzzmmkac9my4auk29j5feumfh77ff")
        );
        assert_json_value_fails::<SUT>(json!("super invalid"));
    }

    #[test]
    fn network_id_stokenet() {
        let a: SUT =
            "validator_tdx_2_1sdatqsl6rx05yy2yvpf6ckfl7x8dluvzkcyljkn0x4lxkgucc0xz2w"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Stokenet);
    }

    #[test]
    fn network_id_mainnet() {
        let a: SUT =
            "validator_rdx1sd5368vqdmjk0y2w7ymdts02cz9c52858gpyny56xdvzuheepdeyy0"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Mainnet);
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::{new_resource_address, EntityAddress};

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ValidatorAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "validator_rdx1sd5368vqdmjk0y2w7ymdts02cz9c52858gpyny56xdvzuheepdeyy0";
        let address = new_validator_address(b32.to_owned()).unwrap();
        assert_eq!(SUT::try_from_bech32(b32).unwrap(), address);
        assert_eq!(validator_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(validator_address_bech32_address(&address), b32);
    }

    #[test]
    fn placeholder() {
        assert_eq!(
            new_validator_address_placeholder_mainnet(),
            SUT::placeholder_mainnet()
        );

        assert_eq!(
            new_validator_address_placeholder_mainnet_other(),
            SUT::placeholder_mainnet_other()
        );

        assert_eq!(
            new_validator_address_placeholder_stokenet(),
            SUT::placeholder_stokenet()
        );

        assert_eq!(
            new_validator_address_placeholder_stokenet_other(),
            SUT::placeholder_stokenet_other()
        );
    }
}
