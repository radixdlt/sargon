use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record, derive_more::Display)]
#[display("{}", self.instructions_string())] // TODO add blobs to Display
pub struct TransactionManifest {
    secret_magic: TransactionManifestSecretMagic,
}

impl TransactionManifest {
    pub fn new(
        instructions_string: impl AsRef<str>,
        network_id: NetworkID,
        blobs: Blobs,
    ) -> Result<Self> {
        Instructions::new(instructions_string, network_id).map(|instructions| {
            Self {
                secret_magic: TransactionManifestSecretMagic {
                    instructions,
                    blobs,
                },
            }
        })
    }

    pub fn with_instructions_and_blobs(
        instructions: Instructions,
        blobs: Blobs,
    ) -> Self {
        Self {
            secret_magic: TransactionManifestSecretMagic::new(
                instructions,
                blobs,
            ),
        }
    }
}

impl TransactionManifest {
    pub(crate) fn empty(network_id: NetworkID) -> Self {
        Self {
            secret_magic: TransactionManifestSecretMagic {
                instructions: Instructions::empty(network_id),
                blobs: Blobs::default(),
            },
        }
    }
}

impl From<TransactionManifestSecretMagic> for TransactionManifest {
    fn from(value: TransactionManifestSecretMagic) -> Self {
        Self {
            secret_magic: value,
        }
    }
}

impl TransactionManifest {
    pub(crate) fn scrypto_manifest(&self) -> ScryptoTransactionManifest {
        ScryptoTransactionManifest {
            instructions: self.instructions().clone(),
            blobs: self.secret_magic.blobs.clone().into(),
        }
    }
}

impl From<TransactionManifest> for ScryptoTransactionManifest {
    fn from(value: TransactionManifest) -> Self {
        value.scrypto_manifest()
    }
}

impl TryFrom<(ScryptoTransactionManifest, NetworkID)> for TransactionManifest {
    type Error = CommonError;
    fn try_from(
        value: (ScryptoTransactionManifest, NetworkID),
    ) -> Result<Self> {
        let scrypto_manifest = value.0;
        let network_id = value.1;
        let instructions = Instructions::try_from((
            scrypto_manifest.clone().instructions.as_ref(),
            network_id,
        ))?;
        let value = Self {
            secret_magic: TransactionManifestSecretMagic::new(
                instructions,
                scrypto_manifest.blobs.clone(),
            ),
        };
        assert_eq!(value.scrypto_manifest(), scrypto_manifest);
        Ok(value)
    }
}

impl TransactionManifest {
    pub fn sargon_built(
        builder: ScryptoManifestBuilder,
        network_id: NetworkID,
    ) -> Self {
        let scrypto_manifest = builder.build();
        Self::try_from((scrypto_manifest, network_id)).expect(
            "Sargon should not build manifest with too nested SBOR depth.",
        )
    }
}

impl TransactionManifest {
    pub(crate) fn instructions(&self) -> &Vec<ScryptoInstruction> {
        self.secret_magic.instructions()
    }

    pub(crate) fn blobs(&self) -> &Blobs {
        &self.secret_magic.blobs
    }

    pub fn instructions_string(&self) -> String {
        self.secret_magic.instructions.instructions_string()
    }

    pub fn summary(&self) -> ManifestSummary {
        let ret_summary = RET_summary(&self.scrypto_manifest());
        ManifestSummary::from((ret_summary, self.network_id()))
    }

    pub fn network_id(&self) -> NetworkID {
        self.secret_magic.instructions.network_id
    }

    pub fn involved_resource_addresses(&self) -> Vec<ResourceAddress> {
        let (addresses, _) = RET_ins_extract_addresses(self.instructions());
        addresses
            .into_iter()
            .filter_map(|a| {
                ResourceAddress::new(*a.as_node_id(), self.network_id()).ok()
            })
            .collect_vec()
    }

    pub fn involved_pool_addresses(&self) -> Vec<PoolAddress> {
        let (addresses, _) = RET_ins_extract_addresses(self.instructions());
        addresses
            .into_iter()
            .filter_map(|a| {
                PoolAddress::new(*a.as_node_id(), self.network_id()).ok()
            })
            .collect_vec()
    }
}

impl HasSampleValues for TransactionManifest {
    fn sample() -> Self {
        TransactionManifestSecretMagic::sample().into()
    }

    fn sample_other() -> Self {
        TransactionManifestSecretMagic::sample_other().into()
    }
}

#[allow(unused)]
impl TransactionManifest {
    pub(crate) fn sample_mainnet_without_lock_fee() -> Self {
        TransactionManifestSecretMagic::sample_mainnet_without_lock_fee().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use std::collections::BTreeMap;

    impl FromStr for TransactionManifest {
        type Err = crate::CommonError;

        fn from_str(s: &str) -> Result<Self> {
            Self::new(s, NetworkID::Simulator, Blobs::default())
        }
    }

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

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
    fn sample_string_roundtrip() {
        let sut = SUT::sample();
        assert_eq!(sut.clone(), sut.clone());
        instructions_eq(
            sut.clone().secret_magic.instructions,
            Instructions::sample_mainnet_instructions_string(),
        );
        assert_eq!(sut.instructions().len(), 4);
    }

    #[test]
    fn sample_other_string_roundtrip() {
        let sut = SUT::sample_other();
        assert_eq!(sut.clone(), sut.clone());
        instructions_eq(
            sut.clone().secret_magic.instructions,
            Instructions::sample_other_simulator_instructions_string(),
        );
        assert_eq!(sut.instructions().len(), 8);
    }

    #[test]
    fn scrypto_roundtrip() {
        let ins = vec![
            ScryptoInstruction::DropAllProofs,
            ScryptoInstruction::DropAuthZoneProofs,
        ];
        let scrypto = ScryptoTransactionManifest {
            instructions: ins.clone(),
            blobs: BTreeMap::new(),
        };

        let sut = SUT::with_instructions_and_blobs(
            Instructions::new_unchecked(ins, NetworkID::Mainnet),
            Blobs::default(),
        );
        assert_eq!(scrypto.clone(), sut.clone().into());
        assert_eq!(sut.scrypto_manifest(), scrypto);
    }

    #[test]
    fn new_from_instructions_string() {
        let instructions_str = r#"CALL_METHOD
        Address("account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q")
        "lock_fee"
        Decimal("500");
                "#;

        assert_eq!(
            SUT::new(instructions_str, NetworkID::Simulator, Blobs::default())
                .unwrap()
                .instructions()
                .len(),
            1
        );
    }

    #[test]
    fn new_from_instructions_string_wrong_network_id_sim_main() {
        let instructions_str = r#"CALL_METHOD
        Address("account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q")
        "lock_fee"
        Decimal("500");
                "#;

        assert_eq!(
            SUT::new(instructions_str, NetworkID::Mainnet, Blobs::default()),
            Err(CommonError::InvalidInstructionsWrongNetwork {
                found_in_instructions: NetworkID::Simulator,
                specified_to_instructions_ctor: NetworkID::Mainnet
            })
        );
    }

    #[test]
    fn new_from_instructions_string_wrong_network_id_main_sim() {
        let instructions_str = r#"CALL_METHOD
        Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
        "lock_fee"
        Decimal("500");
                "#;

        assert_eq!(
            SUT::new(instructions_str, NetworkID::Stokenet, Blobs::default()),
            Err(CommonError::InvalidInstructionsWrongNetwork {
                found_in_instructions: NetworkID::Mainnet,
                specified_to_instructions_ctor: NetworkID::Stokenet
            })
        );
    }

    #[test]
    fn network_id() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
        assert_eq!(SUT::sample_other().network_id(), NetworkID::Simulator);
    }

    #[test]
    fn manifest_summary_simple() {
        let manifest = SUT::sample();
        let summary = manifest.summary();
        pretty_assertions::assert_eq!(
            summary,
            ManifestSummary::new(
                [AccountAddress::sample()],
                [AccountAddress::sample_other()],
                [AccountAddress::sample()],
                [],
            )
        );
    }

    #[test]
    fn manifest_summary_multi_account_resources_transfer() {
        let a = AccountAddress::from("account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q");

        let manifest = SUT::sample_other();
        let summary = manifest.summary();
        pretty_assertions::assert_eq!(
            summary,
            ManifestSummary::new(
                [
                    a
                ],
                [
                    AccountAddress::from("account_sim1c8mulhl5yrk6hh4jsyldps5sdrp08r5v9wusupvzxgqvhlp4c4nwjz"),
                    AccountAddress::from("account_sim1c8s2hass5g62ckwpv78y8ykdqljtetv4ve6etcz64gveykxznj36tr"),
                    AccountAddress::from("account_sim1c8ct6jdcwqrg3gzskyxuy0z933fe55fyjz6p56730r95ulzwl3ppva"),
                ],
                [a],
                []
            )
        );
    }

    #[test]
    fn involved_resource_addresses() {
        let manifest = SUT::sample();
        let resources = manifest.involved_resource_addresses();
        assert_eq!(resources[0].address(), "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd");
    }

    #[test]
    fn involved_pool_addresses() {
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "redeem_from_bi_pool.rtm"
        ));
        let sut = SUT::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();
        let pool_addresses = sut.involved_pool_addresses();
        assert_eq!(pool_addresses, ["pool_tdx_2_1ckfjmjswvvf6y635f8l89uunu9cwgnglhqdk8627wrpf8ultdx2vc3"].into_iter().map(PoolAddress::from).collect_vec());
    }
}
