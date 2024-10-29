use crate::prelude::*;
use radix_common::prelude::ManifestBucket;
use radix_transactions::manifest::KnownManifestObjectNames;

#[derive(Clone, Debug, PartialEq, Eq, derive_more::Display)]
#[display("{}", self.manifest_string())]
pub struct TransactionManifestV2 {
    pub instructions: InstructionsV2,
    pub blobs: Blobs,
    pub children: ChildIntents,
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
                instructions,
                blobs,
                children,
            },
        )
    }

    pub fn with_instructions_and_blobs_and_children(
        instructions: InstructionsV2,
        blobs: Blobs,
        children: ChildIntents,
    ) -> Self {
        Self {
            instructions,
            blobs,
            children,
        }
    }
}

impl TransactionManifestV2 {
    pub(crate) fn empty(network_id: NetworkID) -> Self {
        Self {
            instructions: InstructionsV2::empty(network_id),
            blobs: Blobs::default(),
            children: ChildIntents::empty(),
        }
    }
}

impl TransactionManifestV2 {
    pub fn scrypto_manifest(&self) -> ScryptoTransactionManifestV2 {
        ScryptoTransactionManifestV2 {
            instructions: self.instructions().clone(),
            blobs: self.blobs.clone().into(),
            children: self.children.clone().into(),
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
            instructions,
            blobs: scrypto_manifest.blobs.clone().into(),
            children: (scrypto_manifest.children.clone(), network_id).into(),
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
        self.instructions.instructions()
    }

    pub fn manifest_string(&self) -> String {
        manifest_v2_string_from(self.scrypto_manifest(), self.instructions.network_id).expect("Should never fail, because should never have allowed invalid manifest.")
    }

    pub fn instructions_string(&self) -> String {
        self.instructions.instructions_string()
    }

    pub fn summary(&self) -> Result<ManifestSummary> {
        let summary = RET_statically_analyze_v2(&self.scrypto_manifest())
            .map_err(|e| {
                error!(
                    "Failed to get execution summary from RET, error: {:?}",
                    e
                );
                CommonError::FailedToGenerateManifestSummary {
                    underlying: format!("{:?}", e),
                }
            })?;
        Ok(ManifestSummary::from((summary, self.network_id())))
    }

    pub fn network_id(&self) -> NetworkID {
        self.instructions.network_id
    }

    pub fn involved_resource_addresses(&self) -> Vec<ResourceAddress> {
        let (addresses, _) =
            RET_ins_extract_addresses_v2(self.instructions.as_slice());
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
        Self {
            instructions: InstructionsV2::sample(),
            blobs: Blobs::default(),
            children: ChildIntents::empty(),
        }
    }

    fn sample_other() -> Self {
        Self {
            instructions: InstructionsV2::sample_other(),
            blobs: Blobs::default(),
            children: ChildIntents::empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use radix_rust::hashmap;
    use radix_transactions::manifest::{
        CallMethod, DropAllProofs, DropAuthZoneProofs,
    };
    use radix_transactions::model::InstructionV1;
    use sbor::ValueKind as ScryptoValueKind;

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
        instructions_eq(
            sut.clone().instructions.to_string(),
            InstructionsV2::sample_mainnet_instructions_string(),
        );
        assert_eq!(sut.instructions().len(), 4);
    }

    #[test]
    fn sample_other_string_roundtrip() {
        let sut = SUT::sample_other();
        assert_eq!(sut.clone(), sut.clone());
        instructions_eq(
            sut.clone().instructions.to_string(),
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
            InstructionsV2::new_unchecked(ins, network_id),
            Blobs::default(),
            (children, network_id).into(),
        );
        assert_eq!(scrypto.clone(), sut.clone().into());
        assert_eq!(sut.scrypto_manifest(), scrypto);
    }

    #[test]
    fn try_from_scrypto() {
        let instructions = vec![
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
            instructions: instructions.clone(),
            blobs: Default::default(),
            children,
            object_names: Default::default(),
        };

        let result = SUT::try_from((scrypto.clone(), NetworkID::Mainnet));
        assert!(result.is_ok());
    }

    #[test]
    fn manifest_string() {
        let ins: Vec<ScryptoInstructionV2> = vec![
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
            children,
            object_names: Default::default(),
        };
        let network_id = NetworkID::Simulator;

        let result = manifest_v2_string_from(scrypto, network_id);
        pretty_assertions::assert_eq!(
            result.unwrap(),
            r#"USE_CHILD
    NamedIntent("intent1")
    Intent("subtxid_sim1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6svfetg7")
;
USE_CHILD
    NamedIntent("intent2")
    Intent("subtxid_sim1v7wlh0dpd5lev6w6s4f2kev562cygmgrm9kqw6swe8w92r4yr7ksuk9pw5")
;
DROP_ALL_PROOFS;
DROP_AUTH_ZONE_PROOFS;
"#
        );
    }

    #[test]
    fn manifest_v2_string_failure() {
        let invalid_value = ScryptoManifestValue::Tuple {
            fields: vec![ScryptoManifestValue::Array {
                element_value_kind: ScryptoValueKind::U8,
                elements: vec![
                    ScryptoManifestValue::U8 { value: 1 },
                    ScryptoManifestValue::U16 { value: 2 },
                ],
            }],
        };
        let dummy_address = ComponentAddress::with_node_id_bytes(
            &[0xffu8; 29],
            NetworkID::Stokenet,
        );
        let invalid_instruction =
            ScryptoInstructionV2::CallMethod(CallMethod {
                address: TryInto::<ScryptoDynamicComponentAddress>::try_into(
                    &dummy_address,
                )
                .unwrap()
                .into(),
                method_name: "dummy".to_owned(),
                args: invalid_value,
            });
        let scrypto_manifest = ScryptoTransactionManifestV2 {
            instructions: vec![invalid_instruction],
            blobs: Default::default(),
            children: Default::default(),
            object_names: Default::default(),
        };
        let network_id = NetworkID::Mainnet;

        let result = manifest_v2_string_from(scrypto_manifest, network_id);
        assert_eq!(
            result,
            Err(CommonError::InvalidManifestFailedToDecompile {
                underlying: "FormattingError(Error)".to_string(),
            })
        )
    }

    #[test]
    fn new_from_instructions_string() {
        let instructions_str = "CALL_METHOD\n    Address(\"account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q\")\n    \"lock_fee\"\n    Decimal(\"500\")\n;\n";

        assert_eq!(
            SUT::new(
                instructions_str,
                NetworkID::Simulator,
                Blobs::default(),
                ChildIntents::empty()
            )
            .unwrap()
            .instructions_string(),
            instructions_str
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
                [],
                [AccountAddress::sample()],
                [AccountAddress::sample_other()],
                [],
                [AccountAddress::sample()],
                [],
                Vec::<_>::sample(),
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
                [],
                [
                    a
                ],
                [
                    AccountAddress::from("account_sim1c8mulhl5yrk6hh4jsyldps5sdrp08r5v9wusupvzxgqvhlp4c4nwjz"),
                    AccountAddress::from("account_sim1c8s2hass5g62ckwpv78y8ykdqljtetv4ve6etcz64gveykxznj36tr"),
                    AccountAddress::from("account_sim1c8ct6jdcwqrg3gzskyxuy0z933fe55fyjz6p56730r95ulzwl3ppva"),
                ],
                [],
                [a],
                [],
                Vec::<_>::sample(),
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

    #[test]
    fn sargon_built() {
        let builder = ScryptoTransactionManifestV2Builder::new_v2()
            .lock_fee_from_faucet();

        assert_eq!(
            SUT::sargon_built(builder, NetworkID::Mainnet,).manifest_string(),
            "CALL_METHOD\n    Address(\"component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet\")\n    \"lock_fee\"\n    Decimal(\"5000\")\n;\n",
        )
    }
}
