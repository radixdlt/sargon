use crate::prelude::*;

decl_address!(
    /// Addresses identifying an asset, either fungible (Token) or non_fungible (NFT), on the Radix network, e.g.
    /// `"resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"`
    /// Being the unique identifier of the Radix Token, the Rad, on mainnet.
    ///
    /// There are fundamentally two different sub-types ([Scrypto's `EntityType`][entt]) of ResourceAddresses:
    /// * GlobalFungibleResourceManager
    /// * GlobalNonFungibleResourceManager
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalResourceAddress`][ret].
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L236-L239
    resource => [
        ScryptoEntityType::GlobalFungibleResourceManager,
        ScryptoEntityType::GlobalNonFungibleResourceManager
    ]
);

impl ResourceAddress {
    pub fn is_fungible(&self) -> bool {
        self.node_id.is_global_fungible_resource_manager()
    }

    pub fn is_non_fungible(&self) -> bool {
        self.node_id.is_global_non_fungible_resource_manager()
    }

    pub fn xrd_on_network(id: NetworkID) -> Self {
        Self::new_from_node_id(XRD, id)
            .expect("Should never fail to get XRD on network.")
    }

    pub fn is_xrd_on_network(&self, id: NetworkID) -> bool {
        self == &Self::xrd_on_network(id)
    }
}

impl TryFrom<(ScryptoManifestResourceAddress, NetworkID)> for ResourceAddress {
    type Error = CommonError;
    fn try_from(
        (address, n): (ScryptoManifestResourceAddress, NetworkID),
    ) -> Result<Self> {
        match address {
            ScryptoManifestResourceAddress::Static(resource_address) => {
                Ok(ResourceAddress::from((resource_address, n)))
            }
            _ => Err(CommonError::NamedAddressesAreNotSupported),
        }
    }
}

impl HasSampleValues for ResourceAddress {
    /// The RAD on mainnet
    fn sample() -> Self {
        Self::sample_mainnet_xrd()
    }

    /// Candy by Gumball club on mainnet
    fn sample_other() -> Self {
        Self::sample_mainnet_candy()
    }
}

#[allow(unused)]
impl ResourceAddress {
    pub fn sample_mainnet() -> Self {
        Self::sample_mainnet_xrd()
    }

    pub fn sample_mainnet_other() -> Self {
        Self::sample_mainnet_candy()
    }

    pub fn sample_stokenet() -> Self {
        Self::sample_stokenet_xrd()
    }

    pub fn sample_stokenet_other() -> Self {
        Self::sample_stokenet_gum()
    }
}

impl ResourceAddress {
    /// The RAD on mainnet
    pub fn sample_mainnet_xrd() -> Self {
        "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
            .parse()
            .expect("XRD")
    }

    /// Candy by Gumball club on mainnet
    pub fn sample_mainnet_candy() -> Self {
        "resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j"
            .parse()
            .expect("Candy")
    }

    pub fn sample_mainnet_nft_gc_membership() -> Self {
        "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa"
            .parse()
            .expect("GC Membership")
    }

    pub fn sample_mainnet_nft_other() -> Self {
        "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd"
            .parse()
            .expect("Valid Scorpion NFT Global ID")
    }

    pub fn sample_stokenet_xrd() -> Self {
        "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc"
            .parse()
            .expect("XRD")
    }

    pub fn sample_stokenet_gum() -> Self {
        "resource_tdx_2_1t4kep9ldg9t0cszj78z6fcr2zvfxfq7muetq7pyvhdtctwxum90scq"
            .parse()
            .expect("Gum")
    }

    pub fn sample_stokenet_gc_tokens() -> Self {
        "resource_tdx_2_1thqcgjw37fjgycpvqr52nx4jcsdeuq75mf2nywme07kzsuds9a4psp"
            .parse()
            .expect("GC Tokens")
    }

    pub fn sample_stokenet_candy() -> Self {
        "resource_tdx_2_1tk30vj4ene95e3vhymtf2p35fzl29rv4us36capu2rz0vretw9gzr3"
            .parse()
            .expect("Candy")
    }

    pub fn sample_stokenet_nft_gc_membership() -> Self {
        "resource_tdx_2_1ng88qk08hrgmad30rzdxpyx779yuta4cwcjc3gstk60jhachsv94g9"
            .parse()
            .expect("GC membership")
    }

    pub fn sample_stokenet_nft_abandon() -> Self {
        "resource_tdx_2_1ng6aanl0nw98dgqxtja3mx4kpa8rzwhyt4q22sy9uul0vf9frs528x"
            .parse()
            .expect("Abandon")
    }

    pub fn sample_stokenet_nft_other() -> Self {
        "resource_tdx_2_1ngw6cufaxs5p82kw49juy2yfkt53se76vr0xfsu3tvyduuw6s0y6lc"
            .parse()
            .expect("valid sample value")
    }

    pub fn sample_sim_xrd() -> Self {
        "resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3"
            .parse()
            .expect("valid sample value")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use radix_common::prelude::ManifestNamedAddress;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceAddress;

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
        assert_ne!(SUT::sample_stokenet(), SUT::sample());
        assert_ne!(SUT::sample_sim_xrd(), SUT::sample_stokenet_xrd());
    }

    #[test]
    fn ord() {
        assert!(SUT::sample_mainnet_candy() < SUT::sample_mainnet_xrd());
    }

    #[test]
    fn is_fungible() {
        assert!(SUT::sample_mainnet_nft_gc_membership().is_non_fungible());
        assert!(!SUT::sample_mainnet_nft_gc_membership().is_fungible());

        assert!(!SUT::sample_mainnet_xrd().is_non_fungible());
        assert!(SUT::sample_mainnet_xrd().is_fungible());
    }

    #[test]
    fn is_xrd_on_network() {
        assert!(SUT::sample_mainnet_xrd().is_xrd_on_network(NetworkID::Mainnet));
        assert!(
            SUT::sample_stokenet_xrd().is_xrd_on_network(NetworkID::Stokenet)
        );

        // Not XRD
        assert!(
            !SUT::sample_mainnet_xrd().is_xrd_on_network(NetworkID::Stokenet)
        );
        assert!(
            !SUT::sample_stokenet_xrd().is_xrd_on_network(NetworkID::Mainnet)
        );

        assert!(
            !SUT::sample_mainnet_candy().is_xrd_on_network(NetworkID::Mainnet)
        );
        assert!(
            !SUT::sample_mainnet_candy().is_xrd_on_network(NetworkID::Stokenet)
        );

        assert!(!SUT::sample_stokenet_candy()
            .is_xrd_on_network(NetworkID::Stokenet));
        assert!(
            !SUT::sample_stokenet_candy().is_xrd_on_network(NetworkID::Mainnet)
        );
    }

    #[test]
    fn map_to_network() {
        assert_eq!(
            SUT::sample_mainnet_xrd().map_to_network(NetworkID::Mainnet),
            SUT::sample_mainnet_xrd()
        ); // self
        assert_eq!(
            SUT::sample_stokenet_xrd().map_to_network(NetworkID::Stokenet),
            SUT::sample_stokenet_xrd()
        ); // self
        assert_eq!(
            SUT::sample_sim_xrd().map_to_network(NetworkID::Simulator),
            SUT::sample_sim_xrd()
        ); // self

        // From Mainnet
        assert_eq!(
            SUT::sample_mainnet_xrd().map_to_network(NetworkID::Stokenet),
            SUT::sample_stokenet_xrd()
        );
        assert_eq!(
            SUT::sample_mainnet_xrd().map_to_network(NetworkID::Simulator),
            SUT::sample_sim_xrd()
        );

        // From Stokenet
        assert_eq!(
            SUT::sample_stokenet_xrd().map_to_network(NetworkID::Mainnet),
            SUT::sample_mainnet_xrd()
        );
        assert_eq!(
            SUT::sample_stokenet_xrd().map_to_network(NetworkID::Simulator),
            SUT::sample_sim_xrd()
        );

        // From Sim
        assert_eq!(
            SUT::sample_sim_xrd().map_to_network(NetworkID::Stokenet),
            SUT::sample_stokenet_xrd()
        );
        assert_eq!(
            SUT::sample_sim_xrd().map_to_network(NetworkID::Mainnet),
            SUT::sample_mainnet_xrd()
        );

        assert_eq!(
            SUT::sample_mainnet_xrd().map_to_network(NetworkID::Stokenet),
            SUT::sample_stokenet_xrd()
        );
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                SUT::sample_mainnet_xrd(),
                SUT::sample_mainnet_candy(),
                SUT::sample_mainnet_nft_gc_membership(),
                SUT::sample_stokenet_xrd(),
                SUT::sample_stokenet_gc_tokens(),
                SUT::sample_stokenet_gum(),
                SUT::sample_stokenet_candy(),
                // twice => duplicates should be removed
                SUT::sample_mainnet_xrd(),
                SUT::sample_mainnet_candy(),
                SUT::sample_mainnet_nft_gc_membership(),
                SUT::sample_stokenet_xrd(),
                SUT::sample_stokenet_gc_tokens(),
                SUT::sample_stokenet_gum(),
                SUT::sample_stokenet_candy(),
            ])
            .len(),
            7
        )
    }

    #[test]
    fn formatted_full() {
        assert_eq!(SUT::sample().formatted(AddressFormat::Full), "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd");
    }

    #[test]
    fn formatted_raw() {
        assert_eq!(SUT::sample().formatted(AddressFormat::Raw), "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd");
    }

    #[test]
    fn formatted_default() {
        assert_eq!(
            SUT::sample().formatted(AddressFormat::Default),
            "reso...radxrd"
        );
    }

    #[test]
    fn display() {
        let s = "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{}", a), s);
    }

    #[test]
    fn debug() {
        let s = "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{:?}", a), s);
    }

    #[test]
    fn json_roundtrip() {
        let a = SUT::sample();
        assert_json_value_eq_after_roundtrip(
            &a,
            json!("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"),
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
            json!("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxxx")
        );
        assert_json_value_fails::<SUT>(
            json!("account_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
        );
        assert_json_value_fails::<SUT>(json!("super invalid"));
    }

    #[test]
    fn network_id_stokenet() {
        let a: SUT =
            "resource_tdx_2_1tkckx9fynl9f7756z8wxphq7wce6vk874nuq4f2nnxgh3nzrwhjdlp"
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

    #[test]
    fn xrd_on_mainnet() {
        assert_eq!(SUT::xrd_on_network(NetworkID::Mainnet).to_string(), "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd");
    }

    #[test]
    fn xrd_on_stokenet() {
        assert_eq!(SUT::xrd_on_network(NetworkID::Stokenet).to_string(), "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc");
    }

    #[test]
    fn from_static_scrypto_manifest_resource_address() {
        let expected_address = SUT::sample_mainnet_xrd();
        let address = ScryptoManifestResourceAddress::Static(
            ScryptoResourceAddress::try_from(expected_address.scrypto())
                .unwrap(),
        );
        let network_id = NetworkID::Mainnet;
        assert_eq!(
            SUT::try_from((address, network_id)).unwrap(),
            expected_address
        );
    }

    #[test]
    fn from_named_scrypto_manifest_resource_address() {
        let address =
            ScryptoManifestResourceAddress::Named(ManifestNamedAddress(0));
        let network_id = NetworkID::Mainnet;
        assert_eq!(
            SUT::try_from((address, network_id)),
            Err(CommonError::NamedAddressesAreNotSupported)
        );
    }
}
