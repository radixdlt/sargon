use crate::prelude::*;

#[uniffi::export]
pub fn new_package_address_sample_mainnet() -> PackageAddress {
    PackageAddress::sample_mainnet_faucet()
}

#[uniffi::export]
pub fn new_package_address_sample_mainnet_other() -> PackageAddress {
    PackageAddress::sample_mainnet_royalty()
}

#[uniffi::export]
pub fn new_package_address_sample_stokenet() -> PackageAddress {
    PackageAddress::sample_stokenet_gumball_club()
}

#[uniffi::export]
pub fn new_package_address_sample_stokenet_other() -> PackageAddress {
    PackageAddress::sample_stokenet_other()
}

impl HasSampleValues for PackageAddress {
    fn sample() -> Self {
        Self::sample_mainnet_faucet()
    }

    fn sample_other() -> Self {
        Self::sample_stokenet_gumball_club()
    }
}

impl PackageAddress {
    pub fn sample_mainnet_faucet() -> Self {
        "package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet"
            .parse()
            .expect("Valid Mainnet package sample address")
    }

    pub fn sample_mainnet_royalty() -> Self {
        "package_rdx1pkgxxxxxxxxxryaltyxxxxxxxxx003849573396xxxxxxxxxryalty"
            .parse()
            .expect("Valid Mainnet package sample address")
    }

    pub fn sample_stokenet_gumball_club() -> Self {
        "package_tdx_2_1pkaw4m82c89hy0gk4dwqtqlln6md8anr2ysnrvegxar53mr6nvn5ay"
            .parse()
            .expect("Valid Stokenet package sample address")
    }

    pub fn sample_stokenet_other() -> Self {
        "package_tdx_2_1p4lftg7zjtmvyw5dwv3fg9cxyumlrya03p5uecqdge9thje4nm5qtk"
            .parse()
            .expect("Valid Stokenet package sample address")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PackageAddress;

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
    fn display() {
        let s = "package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{}", a), s);
    }

    #[test]
    fn debug() {
        let s = "package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{:?}", a), s);
    }

    #[test]
    fn json_roundtrip() {
        let a: PackageAddress =
            "package_tdx_2_1pkaw4m82c89hy0gk4dwqtqlln6md8anr2ysnrvegxar53mr6nvn5ay"
                .parse()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &a,
            json!("package_tdx_2_1pkaw4m82c89hy0gk4dwqtqlln6md8anr2ysnrvegxar53mr6nvn5ay"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("package_rdx1p589ehmmvqa2dnw0jaky3kesjdjvln94hzunsqse8k52083hfcjh63"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(
            json!("package_tdx_2_1pkaw4m82c89hy0gk4dwqtqlln6md8anr2ysnrvegxar53mr6nvn5ax")
        );
        assert_json_value_fails::<SUT>(
            json!("account_tdx_2_1pkaw4m82c89hy0gk4dwqtqlln6md8anr2ysnrvegxar53mr6nvn5ay")
        );
        assert_json_value_fails::<SUT>(json!("super invalid"));
    }

    #[test]
    fn network_id_stokenet() {
        let a: SUT =
            "package_tdx_2_1pkaw4m82c89hy0gk4dwqtqlln6md8anr2ysnrvegxar53mr6nvn5ay"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Stokenet);
    }

    #[test]
    fn network_id_mainnet() {
        let a: PackageAddress =
            "package_rdx1p589ehmmvqa2dnw0jaky3kesjdjvln94hzunsqse8k52083hfcjh63"
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
    type SUT = PackageAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "package_rdx1p589ehmmvqa2dnw0jaky3kesjdjvln94hzunsqse8k52083hfcjh63";
        let address = new_package_address(b32.to_owned()).unwrap();
        assert_eq!(package_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(package_address_bech32_address(&address), b32);
    }

    #[test]
    fn new() {
        let s = "package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet";
        let a = SUT::try_from_bech32(s).unwrap();
        let b = new_package_address(s.to_string()).unwrap();
        assert_eq!(b.address(), s);
        assert_eq!(a, b);
    }

    #[test]
    fn hash_of_sample() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_package_address_sample_mainnet(),
                new_package_address_sample_mainnet_other(),
                new_package_address_sample_stokenet(),
                new_package_address_sample_stokenet_other(),
                // duplicates should be removed
                new_package_address_sample_mainnet(),
                new_package_address_sample_mainnet_other(),
                new_package_address_sample_stokenet(),
                new_package_address_sample_stokenet_other(),
            ])
            .len(),
            4
        );
    }
}
