use crate::prelude::*;
use radix_common::prelude::ManifestBucket;
use radix_transactions::manifest::KnownManifestObjectNames;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record, derive_more::Display)]
#[display("{}", self.manifest_string())]
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
            object_names: Default::default(),
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

        // Verify that the manifest can be decompiled and that the instructions are from a validated notarized transaction
        _ = manifest_string_from(scrypto_manifest.clone(), network_id)?;

        assert_eq!(
            value.scrypto_manifest().instructions,
            scrypto_manifest.instructions
        );
        assert_eq!(value.scrypto_manifest().blobs, scrypto_manifest.blobs);
        Ok(value)
    }
}

pub fn manifest_string_from(
    scrypto_manifest: ScryptoTransactionManifest,
    network_id: NetworkID,
) -> Result<String, CommonError> {
    let network_definition = network_id.network_definition();
    scrypto_decompile(&scrypto_manifest, &network_definition).map_err(|e| {
        CommonError::InvalidManifestFailedToDecompile {
            underlying: format!("{:?}", e),
        }
    })
}

impl TryFrom<(UnvalidatedTransactionManifest, NetworkID)>
    for TransactionManifest
{
    type Error = CommonError;
    fn try_from(
        value: (UnvalidatedTransactionManifest, NetworkID),
    ) -> Result<TransactionManifest> {
        TransactionManifest::new(
            value.0.transaction_manifest_string.clone(),
            value.1,
            value.0.blobs.clone(),
        )
    }
}

impl TransactionManifest {
    pub fn sargon_built(
        builder: ScryptoTransactionManifestBuilder,
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

    pub fn manifest_string(&self) -> String {
        manifest_string_from(self.scrypto_manifest(), self.secret_magic.instructions.network_id).expect("Should never fail, because should never have allowed invalid manifest.")
    }

    pub fn instructions_string(&self) -> String {
        self.secret_magic.instructions.instructions_string()
    }

    pub fn summary(&self) -> Option<ManifestSummary> {
        let summary = RET_statically_analyze(&self.scrypto_manifest())?;
        Some(ManifestSummary::from((summary, self.network_id())))
    }

    pub fn network_id(&self) -> NetworkID {
        self.secret_magic.instructions.network_id
    }

    pub fn involved_resource_addresses(&self) -> Vec<ResourceAddress> {
        let (addresses, _) = RET_ins_extract_addresses(
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
    use radix_common::prelude::ManifestBucket;
    use radix_rust::hashmap;
    use radix_transactions::manifest::{
        CallMethod, DropAllProofs, DropAuthZoneProofs,
    };
    use radix_transactions::model::InstructionV1;
    use sbor::ValueKind as ScryptoValueKind;
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
        let ins: Vec<InstructionV1> = vec![
            ScryptoInstruction::DropAllProofs(DropAllProofs),
            ScryptoInstruction::DropAuthZoneProofs(DropAuthZoneProofs),
        ]
        .into();
        let scrypto = ScryptoTransactionManifest {
            instructions: ins.clone(),
            blobs: Default::default(),
            object_names: Default::default(),
        };

        let sut = SUT::with_instructions_and_blobs(
            Instructions::new_unchecked(ins, NetworkID::Mainnet),
            Blobs::default(),
        );
        assert_eq!(scrypto.clone(), sut.clone().into());
        assert_eq!(sut.scrypto_manifest(), scrypto);
    }

    #[test]
    fn manifest_string() {
        let ins: Vec<ScryptoInstruction> = vec![
            ScryptoInstruction::DropAllProofs(DropAllProofs),
            ScryptoInstruction::DropAuthZoneProofs(DropAuthZoneProofs),
        ]
        .into();
        let scrypto = ScryptoTransactionManifest {
            instructions: ins.clone(),
            blobs: Default::default(),
            object_names: Default::default(),
        };
        let network_id = NetworkID::Simulator;

        let result = manifest_string_from(scrypto, network_id);
        pretty_assertions::assert_eq!(
            result.unwrap(),
            r#"DROP_ALL_PROOFS;
DROP_AUTH_ZONE_PROOFS;
"#
        );
    }

    #[test]
    fn manifest_string_failure() {
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
        let invalid_instruction = ScryptoInstruction::CallMethod(CallMethod {
            address: TryInto::<ScryptoDynamicComponentAddress>::try_into(
                &dummy_address,
            )
            .unwrap()
            .into(),
            method_name: "dummy".to_owned(),
            args: invalid_value,
        });
        let scrypto_manifest = ScryptoTransactionManifest {
            instructions: vec![invalid_instruction],
            blobs: Default::default(),
            object_names: Default::default(),
        };
        let network_id = NetworkID::Mainnet;

        let result = manifest_string_from(scrypto_manifest, network_id);
        assert_eq!(
            result,
            Err(CommonError::InvalidManifestFailedToDecompile {
                underlying: "FormattingError(Error)".to_string(),
            })
        )
    }

    #[test]
    fn non_sensical_manifest_does_not_crash() {
        // We are passing in an account address instead of resource address as argument
        // to "withdraw", which does not make any sense. In an earlier version of RET
        // this caused `summary` call to crash. But the PR adding this test bumps RET to
        // a version which does not crash. Does not hurt to keep this unit test around.
        let non_sensical = r#"
CALL_METHOD
  Address("account_tdx_2_1c90qdw5e3t3tjyd8axt3zg9zezhhhymt2mr8y4l0k2285mfwhczhdt")
  "withdraw"
  Address("account_tdx_2_12y0errvzu4caktegxc5v0ug93u5yat9d90k4zdqkcy6n55pt7wlgmq")
  Decimal("0.01")
;
TAKE_FROM_WORKTOP
  Address("resource_tdx_2_1thqru9whuem5sjltshg4q7vj3e22xfh27u6s9xvwecz4fallqjny90")
  Decimal("0.01")
  Bucket("burn")
;

BURN_RESOURCE
  Bucket("burn")
;
        "#;

        let manifest = TransactionManifest::new(
            non_sensical,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();
        let summary = manifest.summary();
        assert!(summary.is_none());
    }

    #[test]
    fn new_from_instructions_string() {
        let instructions_str = r#"CALL_METHOD
        Address("account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q")
        "lock_fee"
        Decimal("500");
                "#;

        assert_eq!(
            SUT::new(instructions_str, NetworkID::Simulator, Blobs::default(),)
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
            SUT::new(instructions_str, NetworkID::Mainnet, Blobs::default(),),
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
            SUT::new(instructions_str, NetworkID::Stokenet, Blobs::default(),),
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
        )
        .unwrap();
        let pool_addresses = sut.involved_pool_addresses();
        assert_eq!(pool_addresses, ["pool_tdx_2_1c5mygu9t8rlfq6j8v2ynrg60ltts2dctsghust8u2tuztrml427830"].into_iter().map(PoolAddress::from).collect_vec());
    }
}
