use crate::prelude::*;

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
#[display("{__inner}")]
pub struct PackageAddress {
    pub(crate) __inner: InnerPackageAddress,
}

#[uniffi::export]
pub fn new_package_address(bech32: String) -> Result<PackageAddress> {
    PackageAddress::try_from_bech32(bech32.as_str())
}

impl HasPlaceholder for PackageAddress {
    fn placeholder() -> Self {
        Self::placeholder_mainnet_gumball_club()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_stokenet_gumball_club()
    }
}

impl PackageAddress {
    pub fn placeholder_mainnet_gumball_club() -> Self {
        "package_rdx1p589ehmmvqa2dnw0jaky3kesjdjvln94hzunsqse8k52083hfcjh63"
            .parse()
            .expect("Valid Mainnet package placeholder address")
    }
    pub fn placeholder_stokenet_gumball_club() -> Self {
        "package_tdx_2_1pkaw4m82c89hy0gk4dwqtqlln6md8anr2ysnrvegxar53mr6nvn5ay"
            .parse()
            .expect("Valid Stokenet package placeholder address")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PackageAddress;


    #[test]
    fn equality() {
        assert_eq!(
            SUT::placeholder(),
            SUT::placeholder()
        );
        assert_eq!(
            SUT::placeholder_other(),
            SUT::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            SUT::placeholder(),
            SUT::placeholder_other()
        );
    }

    #[test]
    fn display() {
        let s = "package_rdx1p589ehmmvqa2dnw0jaky3kesjdjvln94hzunsqse8k52083hfcjh63";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{a}"), s);
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
            json!("resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"),
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
    fn new() {
        let s = "package_rdx1p589ehmmvqa2dnw0jaky3kesjdjvln94hzunsqse8k52083hfcjh63";
        let a = SUT::try_from_bech32(s).unwrap();
        let b = new_package_address(s.to_string()).unwrap();
        assert_eq!(b.address(), s);
        assert_eq!(a, b);
    }
}
