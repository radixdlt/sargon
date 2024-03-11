use crate::prelude::*;

#[uniffi::export]
pub fn new_validator_address_sample_mainnet() -> ValidatorAddress {
    ValidatorAddress::sample_mainnet()
}

#[uniffi::export]
pub fn new_validator_address_sample_mainnet_other() -> ValidatorAddress {
    ValidatorAddress::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_validator_address_sample_stokenet() -> ValidatorAddress {
    ValidatorAddress::sample_stokenet()
}

#[uniffi::export]
pub fn new_validator_address_sample_stokenet_other() -> ValidatorAddress {
    ValidatorAddress::sample_stokenet_other()
}

impl HasSampleValues for ValidatorAddress {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_mainnet_other()
    }
}

impl ValidatorAddress {
    pub fn sample_mainnet() -> Self {
        "validator_rdx1sd5368vqdmjk0y2w7ymdts02cz9c52858gpyny56xdvzuheepdeyy0"
            .parse()
            .expect("Valid sample")
    }

    pub fn sample_mainnet_other() -> Self {
        "validator_rdx1sw5rrhkxs65kl9xcxu7t9yu3k8ptscjwamum4phclk297j6r28g8kd"
            .parse()
            .expect("Valid sample other")
    }

    pub fn sample_stokenet() -> Self {
        "validator_tdx_2_1sdatqsl6rx05yy2yvpf6ckfl7x8dluvzkcyljkn0x4lxkgucc0xz2w".parse().expect("Valid sample")
    }

    pub fn sample_stokenet_other() -> Self {
        "validator_tdx_2_1sdtnujyn3720ymg8lakydkvc5tw4q3zecdj95akdwt9de362mvtd94".parse().expect("Valid sample")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ValidatorAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());

        assert_eq!(SUT::sample_stokenet(), SUT::sample_stokenet());
        assert_eq!(SUT::sample_stokenet_other(), SUT::sample_stokenet_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
        assert_ne!(SUT::sample(), SUT::sample_stokenet());
    }

    #[test]
    fn display() {
        let s = "validator_rdx1sdcmd3ymwzvswgyva8lpknqrzuzzmmkac9my4auk29j5feumfh77fs";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{}", a), s);
    }

    #[test]
    fn debug() {
        let s = "validator_rdx1sdcmd3ymwzvswgyva8lpknqrzuzzmmkac9my4auk29j5feumfh77fs";
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
    fn sample() {
        assert_eq!(
            new_validator_address_sample_mainnet(),
            SUT::sample_mainnet()
        );

        assert_eq!(
            new_validator_address_sample_mainnet_other(),
            SUT::sample_mainnet_other()
        );

        assert_eq!(
            new_validator_address_sample_stokenet(),
            SUT::sample_stokenet()
        );

        assert_eq!(
            new_validator_address_sample_stokenet_other(),
            SUT::sample_stokenet_other()
        );
    }
}
