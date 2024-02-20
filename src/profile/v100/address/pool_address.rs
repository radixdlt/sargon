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
pub struct PoolAddress {
    pub(crate) __inner: InnerPoolAddress,
}

impl HasPlaceholder for PoolAddress {
    fn placeholder() -> Self {
        Self::placeholder_mainnet_bi_pool()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_mainnet_single_pool()
    }
}

impl PoolAddress {
    pub fn placeholder_mainnet_single_pool() -> Self {
        "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w"
            .parse()
            .expect("Placeholder")
    }
    pub fn placeholder_mainnet_bi_pool() -> Self {
        "pool_rdx1c5dkfdtdqvczcwzdyvzeuhddyha768p2q28erden533fty8h68ay6m"
            .parse()
            .expect("Placeholder")
    }
    pub fn placeholder_mainnet_multi_pool() -> Self {
        "pool_rdx1cc7etecr23e77z9aqvq9rg43ndh9jkt7dzmaygg4t8c36z8qe6k47t"
            .parse()
            .expect("Placeholder")
    }

    pub fn placeholder_stokenet_single_pool() -> Self {
        "pool_tdx_2_1c3qzq55xdg6a66kn0qsdnw2zwvvxwljx5m3cp7xcdzeym3kpnzmpcp"
            .parse()
            .expect("Placeholder")
    }
    pub fn placeholder_stokenet_bi_pool() -> Self {
        "pool_tdx_2_1c4ml86h8lvfk7jma0jy0vksh8srcxhmtax8nd3aur29qtd2k2wmlzk"
            .parse()
            .expect("Placeholder")
    }
    pub fn placeholder_stokenet_multi_pool() -> Self {
        "pool_tdx_2_1ce2v6h4qqwuy7m55luappx2u2puutgfs9punuz8lpc33xhfh32gsw3"
            .parse()
            .expect("Placeholder")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PoolAddress;

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
    fn display() {
        let s =
            "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{a}"), s);
    }

    #[test]
    fn json_roundtrip() {
        let a: SUT =
            "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w"
                .parse()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &a,
            json!("pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(json!(
            "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3x"
        ));
        assert_json_value_fails::<SUT>(
            json!("account_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w")
        );
        assert_json_value_fails::<SUT>(json!("super invalid"));
    }

    #[test]
    fn network_id_stokenet() {
        let a: SUT =
            "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Stokenet);
    }

    #[test]
    fn network_id_mainnet() {
        let a: SUT =
            "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
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
    type SUT = PoolAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 =
            "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w";
        let address = new_pool_address(b32.to_owned()).unwrap();
        assert_eq!(pool_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(pool_address_bech32_address(&address), b32);
    }

    #[test]
    fn new() {
        let s =
            "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w";
        let a = SUT::try_from_bech32(s).unwrap();
        let b = new_pool_address(s.to_string()).unwrap();
        assert_eq!(b.address(), s);
        assert_eq!(a, b);
    }
}
