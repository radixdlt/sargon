use prelude::fixture_rtm;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, derive_more::Display)]
#[display("{}", self.manifest_string())]
pub struct SubintentManifest {
    pub instructions: InstructionsV2,
    pub blobs: Blobs,
    pub children: ChildSubintentSpecifiers,
}

impl SubintentManifest {
    pub fn new(
        instructions_string: impl AsRef<str>,
        network_id: NetworkID,
        blobs: Blobs,
        children: ChildSubintentSpecifiers,
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
        children: ChildSubintentSpecifiers,
    ) -> Self {
        Self {
            instructions,
            blobs,
            children,
        }
    }
}

impl StaticallyAnalyzableManifest for SubintentManifest {
    fn summary(&self, network_id: NetworkID) -> Result<ManifestSummary> {
        let summary =
            RET_statically_analyze_subintent_manifest(&self.scrypto_manifest())
                .map_err(map_static_analysis_error)?;

        Ok(ManifestSummary::from((summary, network_id)))
    }
}

impl SubintentManifest {
    #[allow(dead_code)]
    pub(crate) fn empty(network_id: NetworkID) -> Self {
        Self {
            instructions: InstructionsV2::empty(network_id),
            blobs: Blobs::default(),
            children: ChildSubintentSpecifiers::empty(),
        }
    }
}

impl SubintentManifest {
    pub fn scrypto_manifest(&self) -> ScryptoSubintentManifestV2 {
        ScryptoSubintentManifestV2 {
            instructions: self.instructions().clone(),
            blobs: self.blobs.clone().into(),
            children: self.children.clone().into(),
            object_names: Default::default(),
        }
    }
}

impl From<SubintentManifest> for ScryptoSubintentManifestV2 {
    fn from(value: SubintentManifest) -> Self {
        value.scrypto_manifest()
    }
}

impl TryFrom<(ScryptoSubintentManifestV2, NetworkID)> for SubintentManifest {
    type Error = CommonError;
    fn try_from(
        value: (ScryptoSubintentManifestV2, NetworkID),
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
        _ = subintent_manifest_v2_string_from(
            scrypto_manifest.clone(),
            network_id,
        )?;

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

pub fn subintent_manifest_v2_string_from(
    scrypto_manifest: ScryptoSubintentManifestV2,
    network_id: NetworkID,
) -> Result<String, CommonError> {
    let network_definition = network_id.network_definition();
    scrypto_decompile(&scrypto_manifest, &network_definition).map_err(|e| {
        CommonError::InvalidManifestFailedToDecompile {
            underlying: format!("{:?}", e),
        }
    })
}

impl SubintentManifest {
    pub fn sargon_built(
        builder: ScryptoSubintentManifestV2Builder,
        network_id: NetworkID,
    ) -> Self {
        let scrypto_manifest = builder.build();
        Self::try_from((scrypto_manifest, network_id)).expect(
            "Sargon should not build manifest with too nested SBOR depth.",
        )
    }
}

impl SubintentManifest {
    pub(crate) fn instructions(&self) -> &Vec<ScryptoInstructionV2> {
        self.instructions.instructions()
    }

    pub fn blobs(&self) -> &Blobs {
        &self.blobs
    }

    pub fn manifest_string(&self) -> String {
        subintent_manifest_v2_string_from(self.scrypto_manifest(), self.instructions.network_id).expect("Should never fail, because should never have allowed invalid manifest.")
    }

    pub fn instructions_string(&self) -> String {
        self.instructions.instructions_string()
    }

    pub fn summary(&self) -> Result<ManifestSummary> {
        StaticallyAnalyzableManifest::summary(self, self.network_id())
    }

    pub fn as_enclosed(&self) -> Option<TransactionManifestV2> {
        let enclosed_manifest = self.as_enclosed_scrypto()?;
        (enclosed_manifest, self.network_id()).try_into().ok()
    }

    pub fn as_enclosed_scrypto(&self) -> Option<ScryptoTransactionManifestV2> {
        RET_subintent_manifest_as_enclosed(&self.scrypto_manifest())
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
                ResourceAddress::new_from_node_id(a, self.network_id()).ok()
            })
            .collect_vec()
    }

    pub fn involved_pool_addresses(&self) -> Vec<PoolAddress> {
        let (addresses, _) = RET_ins_extract_addresses_v2(self.instructions());
        addresses
            .into_iter()
            .filter_map(|a| {
                PoolAddress::new_from_node_id(a, self.network_id()).ok()
            })
            .collect_vec()
    }
}

impl SubintentManifest {
    pub(crate) fn sample_mainnet_instructions_string() -> String {
        fixture_rtm!("resource_transfer_subintent").to_owned()
    }

    pub fn sample_mainnet_instructions() -> InstructionsV2 {
        InstructionsV2::new(
            Self::sample_mainnet_instructions_string(),
            NetworkID::Mainnet,
        )
        .expect("Valid sample value")
    }

    pub(crate) fn sample_other_simulator_instructions_string() -> String {
        fixture_rtm!("multi_account_resource_transfer_subintent").to_owned()
    }

    pub fn sample_simulator_other_instructions() -> InstructionsV2 {
        InstructionsV2::new(
            Self::sample_other_simulator_instructions_string(),
            NetworkID::Simulator,
        )
        .expect("Valid sample value")
    }
}

impl HasSampleValues for SubintentManifest {
    fn sample() -> Self {
        Self {
            instructions: Self::sample_mainnet_instructions(),
            blobs: Blobs::default(),
            children: ChildSubintentSpecifiers::empty(),
        }
    }

    fn sample_other() -> Self {
        Self {
            instructions: Self::sample_simulator_other_instructions(),
            blobs: Blobs::default(),
            children: ChildSubintentSpecifiers::empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prelude::fixture_rtm;
    use radix_rust::hashmap;
    use radix_transactions::manifest::{
        BlobProvider, CallMethod, DropAllProofs, DropAuthZoneProofs,
    };
    use sbor::ValueKind as ScryptoValueKind;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SubintentManifest;

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
            SUT::sample_mainnet_instructions_string(),
        );
        assert_eq!(sut.instructions().len(), 5);
    }

    #[test]
    fn sample_other_string_roundtrip() {
        let sut = SUT::sample_other();
        assert_eq!(sut.clone(), sut.clone());
        instructions_eq(
            sut.clone().instructions.to_string(),
            SUT::sample_other_simulator_instructions_string(),
        );
        assert_eq!(sut.instructions().len(), 9);
    }

    #[test]
    fn scrypto_roundtrip() {
        let network_id = NetworkID::Mainnet;
        let ins = vec![
            ScryptoInstructionV2::DropAllProofs(DropAllProofs),
            ScryptoInstructionV2::DropAuthZoneProofs(DropAuthZoneProofs),
        ];
        let children = IndexSet::from_iter([
            ScryptoChildSubintentSpecifier {
                hash: SubintentHash::sample().into(),
            },
            ScryptoChildSubintentSpecifier {
                hash: SubintentHash::sample_other().into(),
            },
        ]);
        let scrypto = ScryptoSubintentManifestV2 {
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
        let children = IndexSet::from_iter([
            ScryptoChildSubintentSpecifier {
                hash: SubintentHash::sample().into(),
            },
            ScryptoChildSubintentSpecifier {
                hash: SubintentHash::sample_other().into(),
            },
        ]);
        let scrypto = ScryptoSubintentManifestV2 {
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
        let children = IndexSet::from_iter([
            ScryptoChildSubintentSpecifier {
                hash: SubintentHash::sample().into(),
            },
            ScryptoChildSubintentSpecifier {
                hash: SubintentHash::sample_other().into(),
            },
        ]);
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
        let scrypto_manifest = ScryptoSubintentManifestV2 {
            instructions: vec![invalid_instruction],
            blobs: Default::default(),
            children: Default::default(),
            object_names: Default::default(),
        };
        let network_id = NetworkID::Mainnet;

        let result =
            subintent_manifest_v2_string_from(scrypto_manifest, network_id);
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
                ChildSubintentSpecifiers::empty()
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
                ChildSubintentSpecifiers::empty()
            ),
            Err(CommonError::InvalidInstructionsWrongNetwork {
                found_in_instructions: NetworkID::Simulator.to_string(),
                specified_to_instructions_ctor: NetworkID::Mainnet.to_string()
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
                ChildSubintentSpecifiers::empty()
            ),
            Err(CommonError::InvalidInstructionsWrongNetwork {
                found_in_instructions: NetworkID::Mainnet.to_string(),
                specified_to_instructions_ctor: NetworkID::Stokenet.to_string()
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
                    AccountAddress::sample_other() => AccountDeposits::sample(),
                ),
                [],
                [AccountAddress::sample()],
                [AccountAddress::sample_other()],
                [],
                [AccountAddress::sample()],
                [],
                Vec::<_>::sample(),
                [RetManifestClass::GeneralSubintent],
            )
        );
    }

    #[test]
    fn manifest_summary_multi_account_resources_transfer() {
        let a = AccountAddress::from_str("account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q").unwrap();

        let manifest = SUT::sample_other();
        let summary = manifest.summary().unwrap();
        pretty_assertions::assert_eq!(
            summary,
            ManifestSummary::new(
                hashmap!(
                    a => vec![AccountWithdraw::sample()],
                ),
                hashmap!(
                    AccountAddress::from_str("account_sim1c8mulhl5yrk6hh4jsyldps5sdrp08r5v9wusupvzxgqvhlp4c4nwjz").unwrap() =>
                        AccountDeposits::new_for_test(
                            vec![SimpleResourceBounds::exact_fungible(ResourceAddress::sample_sim_xrd(), 150)],
                            UnspecifiedResources::NonePresent,
                        )
                    ,
                    AccountAddress::from_str("account_sim1c8ct6jdcwqrg3gzskyxuy0z933fe55fyjz6p56730r95ulzwl3ppva").unwrap() =>
                        AccountDeposits::new_for_test(
                            vec![SimpleResourceBounds::exact_fungible(ResourceAddress::sample_sim_xrd(), 50)],
                            UnspecifiedResources::NonePresent,
                        )
                    ,
                    AccountAddress::from_str("account_sim1c8s2hass5g62ckwpv78y8ykdqljtetv4ve6etcz64gveykxznj36tr").unwrap() =>
                        AccountDeposits::new_for_test(
                            vec![SimpleResourceBounds::exact_fungible(ResourceAddress::sample_sim_xrd(), 130)],
                            UnspecifiedResources::NonePresent,
                        ),
                ),
                [],
                [
                    a
                ],
                [
                    AccountAddress::from_str("account_sim1c8mulhl5yrk6hh4jsyldps5sdrp08r5v9wusupvzxgqvhlp4c4nwjz").unwrap(),
                    AccountAddress::from_str("account_sim1c8s2hass5g62ckwpv78y8ykdqljtetv4ve6etcz64gveykxznj36tr").unwrap(),
                    AccountAddress::from_str("account_sim1c8ct6jdcwqrg3gzskyxuy0z933fe55fyjz6p56730r95ulzwl3ppva").unwrap(),
                ],
                [],
                [a],
                [],
                Vec::<_>::sample(),
                [RetManifestClass::GeneralSubintent],
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
        let instructions_string = fixture_rtm!("redeem_from_bi_pool");
        let sut = SUT::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
            ChildSubintentSpecifiers::empty(),
        )
        .unwrap();
        let pool_addresses = sut.involved_pool_addresses();
        assert_eq!(pool_addresses, ["pool_tdx_2_1c5mygu9t8rlfq6j8v2ynrg60ltts2dctsghust8u2tuztrml427830"].into_iter().map(PoolAddress::from_str).map(Result::unwrap).collect_vec());
    }

    #[test]
    fn sargon_built() {
        let builder = ScryptoSubintentManifestV2Builder::new_subintent_v2()
            .lock_fee_from_faucet()
            .yield_to_parent(());

        let manifest = SUT::sargon_built(builder, NetworkID::Mainnet);
        assert_eq!(
            manifest.manifest_string(),
            "CALL_METHOD\n    Address(\"component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet\")\n    \"lock_fee\"\n    Decimal(\"5000\")\n;\nYIELD_TO_PARENT;\n",
        )
    }

    #[test]
    fn open_pre_auth_fungibles_deposit_summary() {
        let manifest_str = fixture_rtm!("open_subintent_fungibles");

        let network = NetworkID::Stokenet.network_definition();
        let man: ScryptoSubintentManifestV2 = scrypto_compile_manifest(
            manifest_str,
            &network,
            BlobProvider::new(),
        )
        .unwrap();

        let manifest: SubintentManifest =
            (man, NetworkID::Stokenet).try_into().unwrap();
        let summary = manifest.summary().unwrap();

        pretty_assertions::assert_eq!(
            summary.account_deposits,
            hashmap! {
                AccountAddress::from_str("account_tdx_2_129rfcz44zxflyaf6d65fdvaqtk5rlvdu8nzek2nz435zknhqure2xl").unwrap() =>
                    AccountDeposits::new_for_test(
                        vec![
                            SimpleResourceBounds::fungible("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::unknown_amount()),
                            SimpleResourceBounds::fungible("resource_tdx_2_1th4gzm9rk5s28yf5gud0a32m082g4x60d6na4ecsccte032y3xu785".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::at_least(6)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1tkd957yt3rwqze7elmzlphfjnmfyzkf9l5rau5ccsx9h2vs9nq3tzp".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::at_most(10)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1t5jcmwqx39ym7fw0hrlpnxwechr0mzlgulcfcye59qm9y9fa5uvdrd".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::between(100, 159)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1t5dapa24l4xvwqtqe2jrdphtn7ga46gw67wr9fwn4gp532myfjqpck".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::exact(3)),
                        ],
                        UnspecifiedResources::MayBePresent,
                    ),
            }
        )
    }

    #[test]
    fn open_pre_auth_non_fungibles_no_certain_ids_deposit_summary() {
        let manifest_str =
            fixture_rtm!("open_subintent_non_fungibles_no_certain_ids");

        let network = NetworkID::Stokenet.network_definition();
        let man: ScryptoSubintentManifestV2 = scrypto_compile_manifest(
            manifest_str,
            &network,
            BlobProvider::new(),
        )
        .unwrap();

        let manifest: SubintentManifest =
            (man, NetworkID::Stokenet).try_into().unwrap();
        let summary = manifest.summary().unwrap();

        pretty_assertions::assert_eq!(
            summary.account_deposits,
            hashmap!(
                AccountAddress::from_str("account_tdx_2_129rfcz44zxflyaf6d65fdvaqtk5rlvdu8nzek2nz435zknhqure2xl").unwrap() =>
                    AccountDeposits::new_for_test(
                        vec![
                            SimpleResourceBounds::non_fungible(
                                "resource_tdx_2_1nfmxggm4plrrmc9ft9qn79g7uehqlhjaszv02dnuk85s0h9xnh3xue".parse::<ResourceAddress>().unwrap(),
                                SimpleNonFungibleResourceBounds::new(Vec::new(), Some(SimpleCountedResourceBounds::unknown_amount())),
                            ),
                            SimpleResourceBounds::non_fungible(
                                "resource_tdx_2_1n2lj0rk7pye8h2cxs347lf70ksyzwaez0mjkssccfthp6m408hfny7".parse::<ResourceAddress>().unwrap(),
                                SimpleNonFungibleResourceBounds::new(Vec::new(), Some(SimpleCountedResourceBounds::at_least(6))),
                            ),
                            SimpleResourceBounds::non_fungible(
                                "resource_tdx_2_1nf8g5dhl6rxvq78j6q3kdxfkl7rweychjzyv848clhezg44rn0qgy5".parse::<ResourceAddress>().unwrap(),
                                SimpleNonFungibleResourceBounds::new(Vec::new(), Some(SimpleCountedResourceBounds::at_most(10))),
                            ),
                            SimpleResourceBounds::non_fungible(
                                "resource_tdx_2_1nfn4gd24pcpnqegcq07mgvz9cea4zryytswn5vmgepnan7tjqedkxp".parse::<ResourceAddress>().unwrap(),
                                SimpleNonFungibleResourceBounds::new(Vec::new(), Some(SimpleCountedResourceBounds::between(100, 159))),
                            ),
                            SimpleResourceBounds::non_fungible(
                                "resource_tdx_2_1nt8pgfd7xj954403vfgkej25g8kcc56ldu4j3akl4vzlcfen6jcfjg".parse::<ResourceAddress>().unwrap(),
                                SimpleNonFungibleResourceBounds::new(Vec::new(), Some(SimpleCountedResourceBounds::exact(3))),
                            ),
                        ],
                        UnspecifiedResources::MayBePresent,
                    ),
            )
        );
    }

    #[test]
    fn open_pre_auth_non_fungibles_with_certain_ids_deposit_summary() {
        let manifest_str =
            fixture_rtm!("open_subintent_non_fungibles_with_certain_ids");

        let network = NetworkID::Stokenet.network_definition();
        let man: ScryptoSubintentManifestV2 = scrypto_compile_manifest(
            manifest_str,
            &network,
            BlobProvider::new(),
        )
        .unwrap();

        let manifest: SubintentManifest =
            (man, NetworkID::Stokenet).try_into().unwrap();
        let summary = manifest.summary().unwrap();

        let certain_ids_sample = vec![
            NonFungibleLocalId::from_str("#0#").unwrap(),
            NonFungibleLocalId::from_str("#1#").unwrap(),
            NonFungibleLocalId::from_str("#2#").unwrap(),
        ];

        let member_card_id =
            NonFungibleLocalId::from_str("<Member_103>").unwrap();

        pretty_assertions::assert_eq!(
            summary.account_deposits,
            hashmap!(
            AccountAddress::from_str("account_tdx_2_129rfcz44zxflyaf6d65fdvaqtk5rlvdu8nzek2nz435zknhqure2xl").unwrap() =>
                AccountDeposits::new_for_test(
                    vec![
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1nfmxggm4plrrmc9ft9qn79g7uehqlhjaszv02dnuk85s0h9xnh3xue".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(vec![member_card_id], Some(SimpleCountedResourceBounds::unknown_amount())),
                        ),
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1n2lj0rk7pye8h2cxs347lf70ksyzwaez0mjkssccfthp6m408hfny7".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(certain_ids_sample.clone(), Some(SimpleCountedResourceBounds::unknown_amount())),
                        ),
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1nf8g5dhl6rxvq78j6q3kdxfkl7rweychjzyv848clhezg44rn0qgy5".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(certain_ids_sample.clone(), None),
                        ),
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1nfn4gd24pcpnqegcq07mgvz9cea4zryytswn5vmgepnan7tjqedkxp".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(certain_ids_sample.clone(), Some(SimpleCountedResourceBounds::at_most(2))),
                        ),
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1nt8pgfd7xj954403vfgkej25g8kcc56ldu4j3akl4vzlcfen6jcfjg".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(certain_ids_sample.clone(), None),
                        ),
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1n2q3kj4sfa6sh45kvau2f08hfhjuls7zcevwl77vjzmgf3sea0uzzu".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(certain_ids_sample.clone(), Some(SimpleCountedResourceBounds::at_most(2))),
                        ),
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1n2rpk9w8d8kzu578jxvqr0dplctfh5clylmyqpu9kvnz7hvceh2mxe".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(certain_ids_sample.clone(), Some(SimpleCountedResourceBounds::at_least(2))),
                        ),
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1ngu8tgxvv26rpmdwxxfd8gclnsnjeew7zdcw2p3genru58a7wkmue4".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(certain_ids_sample.clone(), Some(SimpleCountedResourceBounds::exact(2))),
                        ),
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1nfve52p2wvx0kp0eq3xaznuvwakcu5a6aqtsjqq8x30zk4wkglxmlv".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(certain_ids_sample.clone(), Some(SimpleCountedResourceBounds::between(2, 5))),
                        ),
                    ],
                    UnspecifiedResources::MayBePresent,
                ),
            )
        );
    }

    #[test]
    fn test_multiple_deposits() {
        let manifest_str = fixture_rtm!("open_subintent_mix_multiple_deposits");

        let network = NetworkID::Stokenet.network_definition();
        let man: ScryptoSubintentManifestV2 = scrypto_compile_manifest(
            manifest_str,
            &network,
            BlobProvider::new(),
        )
        .unwrap();

        let manifest: SubintentManifest =
            (man, NetworkID::Stokenet).try_into().unwrap();
        let summary = manifest.summary().unwrap();

        pretty_assertions::assert_eq!(
            summary.account_deposits,
            hashmap!(
                AccountAddress::from_str("account_tdx_2_129rfcz44zxflyaf6d65fdvaqtk5rlvdu8nzek2nz435zknhqure2xl").unwrap() =>
                    AccountDeposits::new_for_test(
                        vec![
                            SimpleResourceBounds::fungible("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::unknown_amount()),
                            SimpleResourceBounds::fungible("resource_tdx_2_1th4gzm9rk5s28yf5gud0a32m082g4x60d6na4ecsccte032y3xu785".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::at_least(6)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1tkd957yt3rwqze7elmzlphfjnmfyzkf9l5rau5ccsx9h2vs9nq3tzp".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::at_most(10)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1t5jcmwqx39ym7fw0hrlpnxwechr0mzlgulcfcye59qm9y9fa5uvdrd".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::between(100, 159)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1t5dapa24l4xvwqtqe2jrdphtn7ga46gw67wr9fwn4gp532myfjqpck".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::exact(3)),

                            SimpleResourceBounds::fungible("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::unknown_amount()),
                            SimpleResourceBounds::fungible("resource_tdx_2_1th4gzm9rk5s28yf5gud0a32m082g4x60d6na4ecsccte032y3xu785".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::at_least(6)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1tkd957yt3rwqze7elmzlphfjnmfyzkf9l5rau5ccsx9h2vs9nq3tzp".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::at_most(10)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1t5jcmwqx39ym7fw0hrlpnxwechr0mzlgulcfcye59qm9y9fa5uvdrd".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::between(100, 159)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1t5dapa24l4xvwqtqe2jrdphtn7ga46gw67wr9fwn4gp532myfjqpck".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::exact(3)),
                        ],
                        UnspecifiedResources::MayBePresent,
                    ),
                    AccountAddress::from_str("account_tdx_2_1288u4aka2dm8787texaeta8ruzhcr7dyckmnck5wt0llrm6x0ak7e4").unwrap() => AccountDeposits::new_for_test(
                        vec![
                            SimpleResourceBounds::fungible("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::unknown_amount()),
                            SimpleResourceBounds::fungible("resource_tdx_2_1th4gzm9rk5s28yf5gud0a32m082g4x60d6na4ecsccte032y3xu785".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::at_least(6)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1tkd957yt3rwqze7elmzlphfjnmfyzkf9l5rau5ccsx9h2vs9nq3tzp".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::at_most(10)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1t5jcmwqx39ym7fw0hrlpnxwechr0mzlgulcfcye59qm9y9fa5uvdrd".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::between(100, 159)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1t5dapa24l4xvwqtqe2jrdphtn7ga46gw67wr9fwn4gp532myfjqpck".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::exact(3)),
                        ],
                        UnspecifiedResources::NonePresent,
                    ),
            )
        )
    }
}
