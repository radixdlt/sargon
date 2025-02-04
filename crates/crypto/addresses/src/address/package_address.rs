use crate::prelude::*;

decl_address!(
    /// The unique address identifying a package - which is a collection of blueprints on Ledger, e.g.:
    /// `"package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet"`
    ///
    /// PackageAddress has [Scrypto's `EntityType`][entt] type `GlobalPackage`.
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalPackageAddress`][ret].
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L241C29-L241C42
    package => [ScryptoEntityType::GlobalPackage]
);

impl PackageAddress {
    pub fn sample_mainnet() -> Self {
        Self::sample_mainnet_faucet()
    }

    pub fn sample_mainnet_other() -> Self {
        Self::sample_mainnet_royalty()
    }

    pub fn sample_stokenet() -> Self {
        Self::sample_stokenet_gumball_club()
    }
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
    use super::*;

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
