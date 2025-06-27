use crate::prelude::*;

decl_address!(
    /// Address to a Validator that secures the network by validating transactions, users can stake to these
    /// validators (Delegated Proof of Stake) by using the Dashboard and sending a TX to the Radix Wallet to sign;
    /// e.g.:
    /// `"validator_rdx1sd5368vqdmjk0y2w7ymdts02cz9c52858gpyny56xdvzuheepdeyy0"`
    ///
    /// A `ValidatorAddress` has the [Scrypto's `EntityType`][entt] `GlobalValidator`.
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalValidatorAddress`][ret].
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L249-L250
    validator => [ScryptoEntityType::GlobalValidator]
);

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
    use super::*;

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
