use crate::prelude::*;

decl_address!(
    /// Addresses to a specific locker, owned by a dApp, holding assets, either fungible or non_fungible,
    /// that can be claimed by destined account addresses.
    /// Identities cannot own assets so they do not have vaults, but Accounts do, e.g.:
    /// e.g.:
    /// `"locker_rdx1tz474x29nxxd4k2p2reete9xyz4apawv63dphxkr00qt23vyju49fq"`
    ///
    /// A `LockerAddress` has the [Scrypto's `EntityType`][entt] `GlobalAccountLocker`.
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalLockerAddress`][ret].
    ///
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/476d779fee08ed1e561ac8fc8730a2a404b7de79/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L262-L265
    locker => [
        ScryptoEntityType::GlobalAccountLocker,
    ]
);

impl HasSampleValues for LockerAddress {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_mainnet_other()
    }
}

impl LockerAddress {
    pub fn sample_mainnet() -> Self {
        "locker_rdx1dqeryv3jxgeryv3jxgeryv3jxgeryv3jxgeryv3jxgeryv3jjs0l6p"
            .parse()
            .expect("Valid sample")
    }

    pub fn sample_mainnet_other() -> Self {
        "locker_rdx1drn4q2zk6dvljehytnhfah330xk7emfznv59rqlps5ayy52d7xkzzz"
            .parse()
            .expect("Valid sample other")
    }

    pub fn sample_stokenet() -> Self {
        "locker_tdx_2_1dzjfnkukmlwzz7m9lcjnxdz8ux8d7mlzfddfggzrwmqqwx7qjqx7zc"
            .parse()
            .expect("Valid sample")
    }

    pub fn sample_stokenet_other() -> Self {
        "locker_tdx_2_1dpcz90djy4vlrcs5hjdyk0h5mxddxn038mamcep96s2va667gulfcv"
            .parse()
            .expect("Valid sample")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LockerAddress;

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
        let s =
            "locker_rdx1dzr6rzjaffmmm46dhe4xvq698ee2rguf68waach0532gx4wf8u4mcd";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{}", a), s);
    }

    #[test]
    fn debug() {
        let s =
            "locker_rdx1drfk4x75hlrx68ac9s43swx2xlhf65yhkj8747dapd99q28f9pp4hg";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{:?}", a), s);
    }

    #[test]
    fn json_roundtrip() {
        let a: SUT =
            "locker_rdx1dzr6rzjaffmmm46dhe4xvq698ee2rguf68waach0532gx4wf8u4mcd"
                .parse()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &a,
            json!("locker_rdx1dzr6rzjaffmmm46dhe4xvq698ee2rguf68waach0532gx4wf8u4mcd"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("locker_rdx1drfk4x75hlrx68ac9s43swx2xlhf65yhkj8747dapd99q28f9pp4hg"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(json!(
            "locker_rdx1drfk4x75hlrx68ac9s4a123mxlhf65yhkj8747dapd99q28f9pp4hg"
        ));
        assert_json_value_fails::<SUT>(
            json!("account_rdx1sdcmd3ymwzvswgyva8lpknqrzuzzmmkac9my4auk29j5feumfh77ff")
        );
        assert_json_value_fails::<SUT>(json!("super invalid"));
    }

    #[test]
    fn network_id_stokenet() {
        let a: SUT =
            "locker_tdx_2_1dz38jrn7k59ja40cmx9e4at23jf03wwyxypalca7dfaem2a4hta6la"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Stokenet);
    }

    #[test]
    fn network_id_mainnet() {
        let a: SUT =
            "locker_rdx1dzh2jw7nh83ftxgkkzznjt3fksp2tdcq2qga3spfypwnjsnlw5v9sk"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Mainnet);
    }
}
