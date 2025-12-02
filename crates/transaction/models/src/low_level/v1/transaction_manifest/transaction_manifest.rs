use radix_common::prelude::ManifestCustomValue;
use radix_transactions::data::to_decimal;
use radix_transactions::manifest::CallMethod;

use radix_engine_interface::blueprints::access_controller::{
    ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT as SCRYPTO_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT,
    ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT as SCRYPTO_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
    ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT as SCRYPTO_ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT,
    ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT as SCRYPTO_ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
};

use radix_engine_interface::blueprints::account::ACCOUNT_LOCK_FEE_IDENT as SCRYPTO_ACCOUNT_LOCK_FEE_IDENT;
use sbor::Value;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, derive_more::Display)]
#[display("{}", self.manifest_string())] // TODO add blobs to Display
pub struct TransactionManifest {
    pub instructions: Instructions,
    pub blobs: Blobs,
}

impl TransactionManifest {
    pub fn new(
        instructions_string: impl AsRef<str>,
        network_id: NetworkID,
        blobs: Blobs,
    ) -> Result<Self> {
        Instructions::new(instructions_string, network_id).map(|instructions| {
            Self {
                instructions,
                blobs,
            }
        })
    }

    pub fn with_instructions_and_blobs(
        instructions: Instructions,
        blobs: Blobs,
    ) -> Self {
        Self {
            instructions,
            blobs,
        }
    }
}

impl TransactionManifest {
    pub fn is_access_controller_timed_recovery_manifest(&self) -> bool {
        let has_initiate_recovery_method = self.has_method_for_ident(
            SCRYPTO_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
        ) || self.has_method_for_ident(
            SCRYPTO_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT,
        );

        let has_recovery_quick_confirmation = self.has_method_for_ident(SCRYPTO_ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT) ||
        self.has_method_for_ident(SCRYPTO_ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT);

        has_initiate_recovery_method && !has_recovery_quick_confirmation
    }

    pub fn extract_fee_payer_info(
        &self,
    ) -> Option<(AccountAddress, Decimal192)> {
        for instruction in self.instructions().iter() {
            print!("{:?}", instruction.clone());
            if let ScryptoInstruction::CallMethod(CallMethod {
                address,
                method_name,
                args: Value::Tuple { fields },
            }) = instruction
            {
                if method_name == SCRYPTO_ACCOUNT_LOCK_FEE_IDENT {
                    if let Some(Value::Custom {
                        value: ManifestCustomValue::Decimal(decimal),
                    }) = fields.first()
                    {
                        return Some((
                            AccountAddress::try_from((
                                *address,
                                self.network_id(),
                            ))
                            .expect(
                                "Should be able decode the account address",
                            ),
                            to_decimal(decimal.clone()).into(),
                        ));
                    }
                }
            }
        }
        None
    }

    fn has_method_for_ident(&self, ident: &str) -> bool {
        self.instructions().iter().any(|inst| match inst {
            ScryptoInstruction::CallMethod(method) => {
                method.method_name == ident
            }
            _ => false,
        })
    }
}

impl TransactionManifest {
    pub fn empty(network_id: NetworkID) -> Self {
        Self {
            instructions: Instructions::empty(network_id),
            blobs: Blobs::default(),
        }
    }
}

impl TransactionManifest {
    pub fn scrypto_manifest(&self) -> ScryptoTransactionManifest {
        ScryptoTransactionManifest {
            instructions: self.instructions.instructions().clone(),
            blobs: self.blobs.clone().into(),
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
            instructions,
            blobs: scrypto_manifest.blobs.clone().into(),
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

impl UnvalidatedTransactionManifest {
    pub fn manifest(
        &self,
        network_id: NetworkID,
    ) -> Result<TransactionManifest> {
        (self.clone(), network_id).try_into()
    }
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
    pub fn instructions(&self) -> &Vec<ScryptoInstruction> {
        &self.instructions
    }

    pub fn blobs(&self) -> &Blobs {
        &self.blobs
    }

    pub fn manifest_string(&self) -> String {
        manifest_string_from(self.scrypto_manifest(), self.instructions.network_id).expect("Should never fail, because should never have allowed invalid manifest.")
    }

    pub fn instructions_string(&self) -> String {
        self.instructions.instructions_string()
    }

    pub fn network_id(&self) -> NetworkID {
        self.instructions.network_id
    }

    pub fn involved_resource_addresses(&self) -> Vec<ResourceAddress> {
        let (addresses, _) =
            RET_ins_extract_addresses(self.instructions.as_slice());
        addresses
            .into_iter()
            .filter_map(|a| {
                ResourceAddress::new_from_node_id(a, self.network_id()).ok()
            })
            .collect_vec()
    }

    pub fn involved_pool_addresses(&self) -> Vec<PoolAddress> {
        let (addresses, _) = RET_ins_extract_addresses(self.instructions());
        addresses
            .into_iter()
            .filter_map(|a| {
                PoolAddress::new_from_node_id(a, self.network_id()).ok()
            })
            .collect_vec()
    }
}

impl HasSampleValues for TransactionManifest {
    fn sample() -> Self {
        Self::with_instructions_and_blobs(
            Instructions::sample(),
            Blobs::default(),
        )
    }

    fn sample_other() -> Self {
        Self::with_instructions_and_blobs(
            Instructions::sample_other(),
            Blobs::default(),
        )
    }
}

#[allow(unused)]
impl TransactionManifest {
    pub fn sample_mainnet_without_lock_fee() -> Self {
        let instructions = Instructions::sample_mainnet_without_lock_fee();
        Self::with_instructions_and_blobs(instructions, Blobs::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prelude::fixture_rtm;
    use radix_transactions::manifest::{
        CallMethod, DropAllProofs, DropAuthZoneProofs,
    };
    use radix_transactions::model::InstructionV1;
    use sbor::ValueKind as ScryptoValueKind;

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
            sut.clone().instructions.to_string(),
            Instructions::sample_mainnet_instructions_string(),
        );
        assert_eq!(sut.instructions().len(), 4);
    }

    #[test]
    fn sample_other_string_roundtrip() {
        let sut = SUT::sample_other();
        assert_eq!(sut.clone(), sut.clone());
        instructions_eq(
            sut.clone().instructions.to_string(),
            Instructions::sample_other_simulator_instructions_string(),
        );
        assert_eq!(sut.instructions().len(), 8);
    }

    #[test]
    fn scrypto_roundtrip() {
        let ins: Vec<InstructionV1> = vec![
            ScryptoInstruction::DropAllProofs(DropAllProofs),
            ScryptoInstruction::DropAuthZoneProofs(DropAuthZoneProofs),
        ];
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
    fn try_from_scrypto() {
        let instructions = vec![
            ScryptoInstruction::DropAllProofs(DropAllProofs),
            ScryptoInstruction::DropAuthZoneProofs(DropAuthZoneProofs),
        ];
        let scrypto = ScryptoTransactionManifest {
            instructions: instructions.clone(),
            blobs: Default::default(),
            object_names: Default::default(),
        };

        let result = SUT::try_from((scrypto.clone(), NetworkID::Mainnet));
        assert!(result.is_ok());
    }

    #[test]
    fn test_manifest_string_from() {
        let ins: Vec<ScryptoInstruction> = vec![
            ScryptoInstruction::DropAllProofs(DropAllProofs),
            ScryptoInstruction::DropAuthZoneProofs(DropAuthZoneProofs),
        ];
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
        // The instruction is invalid because it contains a `ScryptoManifestValue::Array`
        // with mixed element types (`U8` and `U16`), which is not allowed.
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
            SUT::new(instructions_str, NetworkID::Stokenet, Blobs::default(),),
            Err(CommonError::InvalidInstructionsWrongNetwork {
                found_in_instructions: NetworkID::Mainnet.to_string(),
                specified_to_instructions_ctor: NetworkID::Stokenet.to_string()
            })
        );
    }

    #[test]
    fn extract_fee_payer_info() {
        let instructions_str = r#"CALL_METHOD
        Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
        "lock_fee"
        Decimal("500");
                "#;

        let sut =
            SUT::new(instructions_str, NetworkID::Mainnet, Blobs::default())
                .unwrap();
        assert_eq!(
            sut.extract_fee_payer_info(),
            Some(
                (
                    AccountAddress::from_str("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr").unwrap(),
                    Decimal192::new("500".to_string()).unwrap(),
                )
            )
        );
    }

    #[test]
    fn network_id() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
        assert_eq!(SUT::sample_other().network_id(), NetworkID::Simulator);
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
        )
        .unwrap();
        let pool_addresses = sut.involved_pool_addresses();
        assert_eq!(pool_addresses, ["pool_tdx_2_1c5mygu9t8rlfq6j8v2ynrg60ltts2dctsghust8u2tuztrml427830"].into_iter().map(PoolAddress::from_str).map(Result::unwrap).collect_vec());
    }
}
