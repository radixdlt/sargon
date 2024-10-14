use crate::prelude::*;
use radix_common::prelude::ManifestBucket;
use radix_transactions::manifest::KnownManifestObjectNames;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record, derive_more::Display)]
#[display("{}", self.manifest_string())]
pub struct TransactionManifestV2 {
    secret_magic: TransactionManifestSecretMagicV2,
}

impl TransactionManifestV2 {
    pub fn new(
        instructions_string: impl AsRef<str>,
        network_id: NetworkID,
        blobs: Blobs,
        children: ChildIntents,
    ) -> Result<Self> {
        InstructionsV2::new(instructions_string, network_id).map(
            |instructions| Self {
                secret_magic: TransactionManifestSecretMagicV2 {
                    instructions,
                    blobs,
                    children,
                },
            },
        )
    }

    pub fn with_instructions_and_blobs_and_children(
        instructions: InstructionsV2,
        blobs: Blobs,
        children: ChildIntents,
    ) -> Self {
        Self {
            secret_magic: TransactionManifestSecretMagicV2::new(
                instructions,
                blobs,
                children,
            ),
        }
    }
}

impl TransactionManifestV2 {
    pub(crate) fn empty(network_id: NetworkID) -> Self {
        Self {
            secret_magic: TransactionManifestSecretMagicV2 {
                instructions: InstructionsV2::empty(network_id),
                blobs: Blobs::default(),
                children: ChildIntents::empty(),
            },
        }
    }
}

impl From<TransactionManifestSecretMagicV2> for TransactionManifestV2 {
    fn from(value: TransactionManifestSecretMagicV2) -> Self {
        Self {
            secret_magic: value,
        }
    }
}

impl TransactionManifestV2 {
    pub(crate) fn scrypto_manifest(&self) -> ScryptoTransactionManifestV2 {
        ScryptoTransactionManifestV2 {
            instructions: self.instructions().clone(),
            blobs: self.secret_magic.blobs.clone().into(),
            children: self.secret_magic.children.clone().into(),
            object_names: Default::default(),
        }
    }
}

impl From<TransactionManifestV2> for ScryptoTransactionManifestV2 {
    fn from(value: TransactionManifestV2) -> Self {
        value.scrypto_manifest()
    }
}

impl TryFrom<(ScryptoTransactionManifestV2, NetworkID)>
    for TransactionManifestV2
{
    type Error = CommonError;
    fn try_from(
        value: (ScryptoTransactionManifestV2, NetworkID),
    ) -> Result<Self> {
        let scrypto_manifest = value.0;
        let network_id = value.1;
        let instructions = InstructionsV2::try_from((
            scrypto_manifest.clone().instructions.as_ref(),
            network_id,
        ))?;
        let value = Self {
            secret_magic: TransactionManifestSecretMagicV2::new(
                instructions,
                scrypto_manifest.blobs.clone(),
                (scrypto_manifest.children.clone(), network_id).into(),
            ),
        };

        // Verify that the manifest can be decompiled and that the instructions are from a validated notarized transaction
        _ = manifest_v2_string_from(scrypto_manifest.clone(), network_id)?;

        assert_eq!(
            value.scrypto_manifest().instructions,
            scrypto_manifest.instructions
        );
        assert_eq!(value.scrypto_manifest().blobs, scrypto_manifest.blobs);
        assert_eq!(
            value.scrypto_manifest().children,
            scrypto_manifest.children
        );
        Ok(value)
    }
}

pub fn manifest_v2_string_from(
    scrypto_manifest: ScryptoTransactionManifestV2,
    network_id: NetworkID,
) -> Result<String, CommonError> {
    let network_definition = network_id.network_definition();
    scrypto_decompile(&scrypto_manifest, &network_definition).map_err(|e| {
        CommonError::InvalidManifestFailedToDecompile {
            underlying: format!("{:?}", e),
        }
    })
}

impl TransactionManifestV2 {
    pub fn sargon_built(
        builder: ScryptoTransactionManifestV2Builder,
        network_id: NetworkID,
    ) -> Self {
        let scrypto_manifest = builder.build();
        Self::try_from((scrypto_manifest, network_id)).expect(
            "Sargon should not build manifest with too nested SBOR depth.",
        )
    }
}

impl TransactionManifestV2 {
    pub(crate) fn instructions(&self) -> &Vec<ScryptoInstructionV2> {
        self.secret_magic.instructions()
    }

    pub(crate) fn blobs(&self) -> &Blobs {
        &self.secret_magic.blobs
    }

    pub(crate) fn children(&self) -> &ChildIntents {
        &self.secret_magic.children
    }

    pub fn manifest_string(&self) -> String {
        manifest_v2_string_from(self.scrypto_manifest(), self.secret_magic.instructions.network_id).expect("Should never fail, because should never have allowed invalid manifest.")
    }

    pub fn instructions_string(&self) -> String {
        self.secret_magic.instructions.instructions_string()
    }

    pub fn summary(&self) -> Option<ManifestSummary> {
        let summary = RET_statically_analyze_v2(&self.scrypto_manifest())?;
        Some(ManifestSummary::from((summary, self.network_id())))
    }

    pub fn network_id(&self) -> NetworkID {
        self.secret_magic.instructions.network_id
    }

    pub fn involved_resource_addresses(&self) -> Vec<ResourceAddress> {
        let (addresses, _) = RET_ins_extract_addresses_v2(
            self.secret_magic.instructions.secret_magic.0.as_slice(),
        );
        addresses
            .into_iter()
            .filter_map(|a| {
                ResourceAddress::new(*a.as_node_id(), self.network_id()).ok()
            })
            .collect_vec()
    }

    pub fn involved_pool_addresses(&self) -> Vec<PoolAddress> {
        let (addresses, _) = RET_ins_extract_addresses_v2(self.instructions());
        addresses
            .into_iter()
            .filter_map(|a| {
                PoolAddress::new(*a.as_node_id(), self.network_id()).ok()
            })
            .collect_vec()
    }
}

impl HasSampleValues for TransactionManifestV2 {
    fn sample() -> Self {
        TransactionManifestSecretMagicV2::sample().into()
    }

    fn sample_other() -> Self {
        TransactionManifestSecretMagicV2::sample_other().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use radix_rust::hashmap;
    use radix_transactions::manifest::{DropAllProofs, DropAuthZoneProofs};
    use radix_transactions::model::InstructionV1;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifestV2;

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
        instructions_v2_eq(
            sut.clone().secret_magic.instructions,
            InstructionsV2::sample_mainnet_instructions_string(),
        );
        assert_eq!(sut.instructions().len(), 4);
    }

    #[test]
    fn sample_other_string_roundtrip() {
        let sut = SUT::sample_other();
        assert_eq!(sut.clone(), sut.clone());
        instructions_v2_eq(
            sut.clone().secret_magic.instructions,
            InstructionsV2::sample_other_simulator_instructions_string(),
        );
        assert_eq!(sut.instructions().len(), 8);
    }

    #[test]
    fn scrypto_roundtrip() {
        let network_id = NetworkID::Mainnet;
        let ins = vec![
            ScryptoInstructionV2::DropAllProofs(DropAllProofs),
            ScryptoInstructionV2::DropAuthZoneProofs(DropAuthZoneProofs),
        ];
        let children = vec![
            ScryptoChildSubintent {
                hash: SubintentHash::sample().into(),
            },
            ScryptoChildSubintent {
                hash: SubintentHash::sample_other().into(),
            },
        ];
        let scrypto = ScryptoTransactionManifestV2 {
            instructions: ins.clone(),
            blobs: Default::default(),
            children: children.clone(),
            object_names: Default::default(),
        };

        let sut = SUT::with_instructions_and_blobs_and_children(
            InstructionsV2::new_unchecked(ins, network_id.clone()),
            Blobs::default(),
            (children, network_id).into(),
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
            SUT::new(
                instructions_str,
                NetworkID::Simulator,
                Blobs::default(),
                ChildIntents::empty()
            )
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
            SUT::new(
                instructions_str,
                NetworkID::Mainnet,
                Blobs::default(),
                ChildIntents::empty()
            ),
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
            SUT::new(
                instructions_str,
                NetworkID::Stokenet,
                Blobs::default(),
                ChildIntents::empty()
            ),
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
        let summary = manifest.summary().unwrap();
        pretty_assertions::assert_eq!(
            summary,
            ManifestSummary::new(
                hashmap!(
                    AccountAddress::sample() => vec![AccountWithdraw::amount(ResourceAddress::sample(), 1337)],
                ),
                hashmap!(
                    AccountAddress::sample_other() => vec![AccountDeposit::sample()],
                ),
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
        let summary = manifest.summary().unwrap();
        pretty_assertions::assert_eq!(
            summary,
            ManifestSummary::new(
                hashmap!(
                    a => vec![AccountWithdraw::sample()],
                ),
                hashmap!(
                    AccountAddress::from("account_sim1c8mulhl5yrk6hh4jsyldps5sdrp08r5v9wusupvzxgqvhlp4c4nwjz") => vec![
                        AccountDeposit::new(
                            vec![
                                (ResourceAddress::sample_sim_xrd(), SimpleResourceBounds::fungible(SimpleFungibleResourceBounds::exact(Decimal::from(150)))),
                            ]
                            .into_iter()
                            .collect(),
                            UnspecifiedResources::NonePresent,
                        )
                    ],
                    AccountAddress::from("account_sim1c8ct6jdcwqrg3gzskyxuy0z933fe55fyjz6p56730r95ulzwl3ppva") => vec![
                        AccountDeposit::new(
                            vec![
                                (ResourceAddress::sample_sim_xrd(), SimpleResourceBounds::fungible(SimpleFungibleResourceBounds::exact(Decimal::from(50)))),
                            ]
                            .into_iter()
                            .collect(),
                            UnspecifiedResources::NonePresent,
                        )
                    ],
                    AccountAddress::from("account_sim1c8s2hass5g62ckwpv78y8ykdqljtetv4ve6etcz64gveykxznj36tr") => vec![
                        AccountDeposit::new(
                            vec![
                                (ResourceAddress::sample_sim_xrd(), SimpleResourceBounds::fungible(SimpleFungibleResourceBounds::exact(Decimal::from(130)))),
                            ]
                            .into_iter()
                            .collect(),
                            UnspecifiedResources::NonePresent,
                        )
                    ],
                ),
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
            ChildIntents::empty(),
        )
        .unwrap();
        let pool_addresses = sut.involved_pool_addresses();
        assert_eq!(pool_addresses, ["pool_tdx_2_1c5mygu9t8rlfq6j8v2ynrg60ltts2dctsghust8u2tuztrml427830"].into_iter().map(PoolAddress::from).collect_vec());
    }
}
