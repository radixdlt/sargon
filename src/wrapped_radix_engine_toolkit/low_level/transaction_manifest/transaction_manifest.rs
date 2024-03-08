use std::ops::Deref;

use crate::prelude::*;

use radix_engine::transaction::TransactionReceipt as ScryptoTransactionReceipt;
use radix_engine_common::data::scrypto::scrypto_decode;
use radix_engine_common::network::NetworkDefinition as ScryptoNetworkDefinition;
use radix_engine_toolkit::functions::instructions::extract_addresses as RET_ins_extract_addresses;
use radix_engine_toolkit::functions::manifest::{
    execution_summary as RET_execution_summary, summary as RET_summary,
};
use transaction::model::{BlobV1 as ScryptoBlob, BlobsV1 as ScryptoBlobs};

use transaction::{
    manifest::compile as scrypto_compile,
    manifest::decompile as scrypto_decompile,
    manifest::MockBlobProvider as ScryptoMockBlobProvider,
    prelude::{
        InstructionV1 as ScryptoInstruction,
        ManifestBuilder as ScryptoManifestBuilder,
        TransactionManifestV1 as ScryptoTransactionManifest,
    },
};

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record, derive_more::Display)]
#[display("{}", self.instructions_string())] // TODO add blobs
pub struct TransactionManifest {
    pub(crate) secret_magic: TransactionManifestSecretMagic,
}

impl TransactionManifest {
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
    fn scrypto_manifest(&self) -> ScryptoTransactionManifest {
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

impl TransactionManifest {
    pub(crate) fn instructions(&self) -> &Vec<ScryptoInstruction> {
        &self.secret_magic.instructions.secret_magic.0
    }

    pub(crate) fn blobs(&self) -> &Blobs {
        &self.secret_magic.blobs
    }

    pub(crate) fn from_scrypto(
        scrypto_manifest: ScryptoTransactionManifest,
        network_id: NetworkID,
    ) -> Self {
        let value = Self {
            secret_magic: TransactionManifestSecretMagic {
                instructions: Instructions {
                    secret_magic: InstructionsSecretMagic(
                        scrypto_manifest.instructions.clone(),
                    ),
                    network_id,
                },
                blobs: scrypto_manifest.blobs.clone().into(),
            },
        };
        assert_eq!(value.scrypto_manifest(), scrypto_manifest);
        value
    }

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

    pub fn instructions_string(&self) -> String {
        self.secret_magic.instructions.instructions_string()
    }

    pub fn summary(&self) -> ManifestSummary {
        let ret_summary = RET_summary(&self.scrypto_manifest());
        ManifestSummary::from_ret(ret_summary, self.network_id())
    }

    pub fn execution_summary(
        &self,
        encoded_receipt: BagOfBytes, // is: Vec<u8>
    ) -> Result<ExecutionSummary> {
        let receipt: TransactionReceipt = encoded_receipt.try_into()?;
        let ret_execution_summary =
            RET_execution_summary(&self.scrypto_manifest(), &receipt.decoded)
                .map_err(|e| {
                error!(
                    "Failed to get execution summary from RET, error: {:?}",
                    e
                );
                CommonError::FailedToGetRetExecutionSummaryFromManifest
            })?;

        Ok(ExecutionSummary::from((
            ret_execution_summary,
            self.network_id(),
        )))
    }

    pub fn network_id(&self) -> NetworkID {
        self.secret_magic.instructions.network_id
    }

    pub fn resource_addresses_to_refresh(&self) -> Vec<ResourceAddress> {
        let (addresses, _) = RET_ins_extract_addresses(self.instructions());
        addresses
            .into_iter()
            .filter_map(|a| {
                ResourceAddress::new(*a.as_node_id(), self.network_id()).ok()
            })
            .collect_vec()
    }
}

#[uniffi::export]
pub fn new_transaction_manifest_from_instructions_string_and_blobs(
    instructions_string: String,
    network_id: NetworkID,
    blobs: Blobs,
) -> Result<TransactionManifest> {
    TransactionManifest::new(instructions_string, network_id, blobs)
}

#[uniffi::export]
pub fn new_transaction_manifest_sample() -> TransactionManifest {
    TransactionManifest::sample()
}

#[uniffi::export]
pub fn new_transaction_manifest_sample_other() -> TransactionManifest {
    TransactionManifest::sample_other()
}

#[uniffi::export]
pub fn transaction_manifest_to_string(
    manifest: &TransactionManifest,
) -> String {
    // FIXME add blobs
    manifest.instructions_string()
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
        let sut = SUT {
            secret_magic: TransactionManifestSecretMagic::new(
                Instructions {
                    secret_magic: InstructionsSecretMagic(ins),
                    network_id: NetworkID::Mainnet,
                },
                Blobs::default(),
            ),
        };
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
        Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
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
    fn execution_summary() {
        let instructions_string = r#"
        CALL_METHOD
            Address("account_tdx_2_128h2zv5m4mnprnfjxn4nf96pglgx064mut8np26hp7w9mm064es2dn")
            "withdraw"
            Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
            Decimal("123")
        ;
        TAKE_FROM_WORKTOP
            Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
            Decimal("123")
            Bucket("bucket1")
        ;
        CALL_METHOD
            Address("account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf")
            "try_deposit_or_abort"
            Bucket("bucket1")
            Enum<0u8>()
        ;
        "#;
        let manifest = SUT::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let summary = manifest
            .execution_summary(TransactionReceipt::sample().encoded)
            .unwrap();

        assert_eq!(
            summary.detailed_classification,
            vec![
                DetailedManifestClass::Transfer,
                DetailedManifestClass::General
            ]
        );
        assert_eq!(
            summary.addresses_of_account_deposits,
            HashMap::from_iter([
                ("account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf".parse::<AccountAddress>().unwrap(), 
                vec![
                    ResourceIndicator::Fungible {
                        resource_address: ResourceAddress::sample_stokenet_xrd(),
                        indicator: FungibleResourceIndicator::Guaranteed { decimal: 123.into() }
                    }
                ])
            ])
        );

        assert_eq!(
            summary.addresses_of_account_withdraws,
            HashMap::from_iter([
                ("account_tdx_2_128h2zv5m4mnprnfjxn4nf96pglgx064mut8np26hp7w9mm064es2dn".parse::<AccountAddress>().unwrap(), 
                vec![
                    ResourceIndicator::Fungible {
                        resource_address: ResourceAddress::sample_stokenet_xrd(),
                        indicator: FungibleResourceIndicator::Guaranteed { decimal: 123.into() }
                    }
                ])
            ])
        );

        assert_eq!(summary.presented_proofs, Vec::default());
        assert_eq!(summary.encountered_component_addresses, Vec::default());
        assert_eq!(summary.reserved_instructions, Vec::default());
        assert_eq!(summary.newly_created_non_fungibles, Vec::default());
        assert_eq!(summary.new_entities, NewEntities::default());
        assert_eq!(summary.fee_locks, FeeLocks::new(0, 0),);

        assert_eq!(
            summary.fee_summary,
            FeeSummary::new(
                "0.1951564".parse::<Decimal>().unwrap(),
                "0.05126075".parse::<Decimal>().unwrap(),
                "0.16679763507".parse::<Decimal>().unwrap(),
                0
            ),
        );
    }

    #[test]
    fn execution_summary_update_third_party_deposits() {
        let instructions_string = r#"
        CALL_METHOD
        	Address("account_tdx_2_12xx9jz27aa0mqjj8cwhk7pzkhtkthv09yclmurse42hlyme2gegyg2")
        	"set_default_deposit_rule"
        	Enum<1u8>()
        ;
        CALL_METHOD
        	Address("account_tdx_2_12xx9jz27aa0mqjj8cwhk7pzkhtkthv09yclmurse42hlyme2gegyg2")
        	"remove_resource_preference"
        	Address("resource_tdx_2_1thqcgjw37fjgycpvqr52nx4jcsdeuq75mf2nywme07kzsuds9a4psp")
        ;
        CALL_METHOD
        	Address("account_tdx_2_12xx9jz27aa0mqjj8cwhk7pzkhtkthv09yclmurse42hlyme2gegyg2")
        	"remove_authorized_depositor"
        	Enum<1u8>(
        		Address("resource_tdx_2_1ngw6cufaxs5p82kw49juy2yfkt53se76vr0xfsu3tvyduuw6s0y6lc")
        	)
        ;
        CALL_METHOD
        	Address("account_tdx_2_12xx9jz27aa0mqjj8cwhk7pzkhtkthv09yclmurse42hlyme2gegyg2")
        	"add_authorized_depositor"
        	Enum<1u8>(
        		Address("resource_tdx_2_1ng88qk08hrgmad30rzdxpyx779yuta4cwcjc3gstk60jhachsv94g9")
        	)
        ;
        "#;
        let encoded_receipt_hex = "5c22000121062108a000743ba40b000000000000000000000000000000000000000900e1f5050900093d00a000743ba40b000000000000000000000000000000000000000980f0fa02a0aaaa829007e54be700000000000000000000000000000000a080cdc975bc56000000000000000000000000000000000000a080cdc975bc560000000000000000000000000000000000002102080000a0000000a1edccce1bc2d300000000000000000000000000002107097c3a1c0009c9680600a0003014b5599f480100000000000000000000000000000000a00014ced1409d4a0000000000000000000000000000000000a0000000000000000000000000000000000000000000000000a08006371efc862c0100000000000000000000000000000000a00000000000000000000000000000000000000000000000002201012102230c09160b4166746572496e766f6b651e0000000e416c6c6f636174654e6f64654964a70200000c4265666f7265496e766f6b65440300000d436c6f73655375627374617465ef3700000a4372656174654e6f6465941700000844726f704e6f64655d2a000009456d69744576656e74b40800002b4f70656e53756273746174653a3a476c6f62616c46756e6769626c655265736f757263654d616e61676572470700002e4f70656e53756273746174653a3a476c6f62616c4e6f6e46756e6769626c655265736f757263654d616e616765727c1100001b4f70656e53756273746174653a3a476c6f62616c5061636b616765052c0e00294f70656e53756273746174653a3a476c6f62616c5669727475616c456432353531394163636f756e74873c0800264f70656e53756273746174653a3a496e7465726e616c47656e65726963436f6d706f6e656e742d3200000750696e4e6f6465540000000c5265616453756273746174657ef700001b52756e4e6174697665436f64653a3a576f726b746f705f64726f70fe4500002752756e4e6174697665436f64653a3a6164645f617574686f72697a65645f6465706f7369746f721aa100002a52756e4e6174697665436f64653a3a72656d6f76655f617574686f72697a65645f6465706f7369746f7211f400002952756e4e6174697665436f64653a3a72656d6f76655f7265736f757263655f707265666572656e63650bed00002752756e4e6174697665436f64653a3a7365745f64656661756c745f6465706f7369745f72756c655de900001156616c696461746554785061796c6f6164704400001256657269667954785369676e617475726573000000000d5772697465537562737461746590100000230c09030c436f6d6d69744576656e74733b4e00000a436f6d6d69744c6f6773000000002f436f6d6d69745374617465557064617465733a3a476c6f62616c5669727475616c456432353531394163636f756e748e1a0600220001210921012320220a071e860c6318c6318c6c4e1b40cc6318c6318cf7bca52eb54a6a86318c6318c60001230722014000012322220100010702000120074a5c220001210222000121022307a00104a05246a9e5f0af00000000000000000000000000000000009058619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed7220000071e82cc6318c6318c659963ed8c6318c6318cf7e8f5ae8f4a96a6318c6318c6000123072202400001232222010001070000012007265c220001210222000121050a75a40000000000000768074107ff0a64000000000000002200006800012322220101012007245c20072041eede177947d8031a947905fdd9af84250449117c32724adc71af61ab1c63f100012007125c2200012102220101220001220000220000071e5dc18449d1f26482602c00e8a99ab2c41b9e03d4da55323b797fac2871b000012307220100000123222200071e9a1dac713d342813aacea965c22889b2e91867da60de64c3915b08de71da00012307220100000123222200071e9a0e7059e7b8d1beb62f189a6090def149c5f6b8762588a20bb69f2bf71700012307220100000123222200071e0d906318c6318c659a6130cc6318c6318cf7a8ba5295eabf46318c6318c6000123072206440001232222004200012322220041000123222200010001232222004500012322220046000123222200071e518c59095eef5fb04a47c3af6f0456baecbbb1e5263fbe0e19aaaff26f2a000123072206000001232222000600012322220005000123222200400001232222010001070000012007115c220001210222000121012201002200004200012322220101012007205c805dc18449d1f26482602c00e8a99ab2c41b9e03d4da55323b797fac2871b0000120070c5c22000121022200002200004300012322220201012007235c220101809a0e7059e7b8d1beb62f189a6090def149c5f6b8762588a20bb69f2bf71700012007115c2200012102220101220001210022000001012007235c220101809a1dac713d342813aacea965c22889b2e91867da60de64c3915b08de71da000120070c5c2200012102220000220000071e0d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60001230722084400012322220000000123222200430001232222004200012322220041000123222200010001232222004500012322220046000123222200071e0d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c600012307220741000123222200440001232222000000012322220042000123222200010001232222004500012322220046000123222200071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed7000123072201400001232222010001070000012007255c2200012102220001a09fdac047cce15f0100000000000000000000000000000000220000210520800020800020800020800023202101071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed702805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6220001a040a58c52cbe15f010000000000000000000000000000000021012320a0002104a0a05246a9e5f0af0000000000000000000000000000000000a0a05246a9e5f0af0000000000000000000000000000000000a040a58c52cbe15f01000000000000000000000000000000002322a00022000120220400012007035c210000012007035c210000012007035c210000012007035c210020210602210222010220071e518c59095eef5fb04a47c3af6f0456baecbbb1e5263fbe0e19aaaff26f2a2200000c1a53657444656661756c744465706f73697452756c654576656e742007065c210122010002210222010220071e518c59095eef5fb04a47c3af6f0456baecbbb1e5263fbe0e19aaaff26f2a2200000c1d52656d6f76655265736f75726365507265666572656e63654576656e742007225c2101805dc18449d1f26482602c00e8a99ab2c41b9e03d4da55323b797fac2871b002210222010220071e518c59095eef5fb04a47c3af6f0456baecbbb1e5263fbe0e19aaaff26f2a2200000c1e52656d6f7665417574686f72697a65644465706f7369746f724576656e742007255c2101220101809a1dac713d342813aacea965c22889b2e91867da60de64c3915b08de71da02210222010220071e518c59095eef5fb04a47c3af6f0456baecbbb1e5263fbe0e19aaaff26f2a2200000c1b416464417574686f72697a65644465706f7369746f724576656e742007255c2101220101809a0e7059e7b8d1beb62f189a6090def149c5f6b8762588a20bb69f2bf71702210222010220071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed72200000c0c4465706f7369744576656e7420071c5c2101a040a58c52cbe15f010000000000000000000000000000000002210222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c194275726e46756e6769626c655265736f757263654576656e7420071c5c2101a040a58c52cbe15f0100000000000000000000000000000000202100210223202304071e860c6318c6318c6c4e1b40cc6318c6318cf7bca52eb54a6a86318c6318c607230140222201000107020301210122000121012103800d906318c6318c6c4e1b40cc6318c6318cf7bfd5d45f48c686318c6318c6200720d8510877df1d820f4752b3c033baf656f62e0e612731718865d048b9d16300b32201010a0900000000000000071e82cc6318c6318c659963ed8c6318c6318cf7e8f5ae8f4a96a6318c6318c607230240222201000107000301210122000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a03000000000000006822220101012007245c20072041eede177947d8031a947905fdd9af84250449117c32724adc71af61ab1c63f10401210222000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a000000000000000022000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a0100000000000000071e518c59095eef5fb04a47c3af6f0456baecbbb1e5263fbe0e19aaaff26f2a07230340222201000107000301210122000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a00000000000000004222220101012007205c805dc18449d1f26482602c00e8a99ab2c41b9e03d4da55323b797fac2871b00401210222000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a220001078522000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a04000000000000004322220201012007235c220101809a0e7059e7b8d1beb62f189a6090def149c5f6b8762588a20bb69f2bf7170401210222000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a22000107e422000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a060000000000000001012007235c220101809a1dac713d342813aacea965c22889b2e91867da60de64c3915b08de71da0401210222000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a22000107e422000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a0600000000000000071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed707230140222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000232121060222010220071e518c59095eef5fb04a47c3af6f0456baecbbb1e5263fbe0e19aaaff26f2a2200000c1a53657444656661756c744465706f73697452756c654576656e740121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a2c000000000000000222010220071e518c59095eef5fb04a47c3af6f0456baecbbb1e5263fbe0e19aaaff26f2a2200000c1d52656d6f76655265736f75726365507265666572656e63654576656e740121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a2b000000000000000222010220071e518c59095eef5fb04a47c3af6f0456baecbbb1e5263fbe0e19aaaff26f2a2200000c1e52656d6f7665417574686f72697a65644465706f7369746f724576656e740121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a2e000000000000000222010220071e518c59095eef5fb04a47c3af6f0456baecbbb1e5263fbe0e19aaaff26f2a2200000c1b416464417574686f72697a65644465706f7369746f724576656e740121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a2d000000000000000222010220071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed72200000c0c4465706f7369744576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a19000000000000000222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c194275726e46756e6769626c655265736f757263654576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a3a000000000000002201012103202100230a20002102a0000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000220000";

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();
        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

        assert_eq!(sut.fee_locks, FeeLocks::new(0, 0));
        assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.092499".parse::<Decimal>().unwrap(),
                "0.02100205".parse::<Decimal>().unwrap(),
                "0.08459091041".parse::<Decimal>().unwrap(),
                0
            )
        );
        assert_eq!(sut.newly_created_non_fungibles, Vec::default());
        assert_eq!(sut.presented_proofs, Vec::default());

        let acc_g2: AccountAddress = "account_tdx_2_12xx9jz27aa0mqjj8cwhk7pzkhtkthv09yclmurse42hlyme2gegyg2".parse().unwrap();

        assert_eq!(
            sut.addresses_of_accounts_requiring_auth,
            vec![acc_g2.clone()]
        );
        assert_eq!(sut.addresses_of_identities_requiring_auth, Vec::default());
        assert_eq!(sut.detailed_classification.len(), 1);

        let (
            resource_preferences_updates,
            deposit_mode_updates,
            authorized_depositors_added,
            authorized_depositors_removed,
        ) = sut.detailed_classification[0]
            .clone()
            .into_account_deposit_settings_update()
            .unwrap();

        assert_eq!(sut.encountered_component_addresses, Vec::default());
        assert_eq!(
            resource_preferences_updates,
            HashMap::<
                AccountAddress,
                HashMap<ResourceAddress, ResourcePreferenceUpdate>,
            >::from_iter([(
                acc_g2.clone(),
                HashMap::<_, _>::from_iter([(
                    ResourceAddress::sample_stokenet_gc_tokens(),
                    ResourcePreferenceUpdate::Remove
                )])
            )])
        );

        assert_eq!(
            deposit_mode_updates,
            HashMap::<AccountAddress, DepositRule>::from_iter([(
                acc_g2.clone(),
                DepositRule::DenyAll
            )])
        );

        assert_eq!(
            authorized_depositors_added,
            HashMap::<AccountAddress, Vec<ResourceOrNonFungible>>::from_iter([
                (
                    acc_g2.clone(),
                    vec![ResourceOrNonFungible::Resource {
                        value:
                            ResourceAddress::sample_stokenet_nft_gc_membership()
                    }]
                )
            ])
        );

        assert_eq!(
            authorized_depositors_removed,
            HashMap::<AccountAddress, Vec<ResourceOrNonFungible>>::from_iter([
                (
                    acc_g2.clone(),
                    vec![ResourceOrNonFungible::Resource {
                        value: ResourceAddress::sample_stokenet_nft_other()
                    }]
                )
            ])
        );
    }

    #[test]
    fn execution_summary_create_single_fungible_resource() {
        let instructions_string = r#"
    CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
        Enum<0u8>()									# Owner role
        true										# Whether the engine should track supply
        10u8										# Divisibility (between 0u8 and 18u8)
        Decimal("100000")							# Initial supply
        Tuple(
            Enum<0u8>(),							# None | Mint Roles (if None: defaults to DenyAll, DenyAll)
            Enum<0u8>(),							# None | Burn Roles (if None: defaults to DenyAll, DenyAll)
            Enum<0u8>(),							# None | Freeze Roles (if None: defaults to DenyAll, DenyAll)
            Enum<0u8>(),							# None | Recall Roles (if None: defaults to DenyAll, DenyAll)
            Enum<0u8>(),							# None | Withdraw Roles (if None: defaults to AllowAll, DenyAll)
            Enum<0u8>()								# None | Deposit Roles (if None: defaults to AllowAll, DenyAll)
        )
        Tuple(										# Metadata initialization
            Map<String, Tuple>(						# Initial metadata values
                "name" => Tuple(
                    Enum<1u8>(
                        Enum<0u8>("MyResource")		# Resource NAME
                    ),
                    false							# Locked
                ),
                "description" => Tuple(				# Resource DESCRIPTION
                    Enum<1u8>(
                        Enum<0u8>(
                            "A
                            very
                            innovative
                            and
                            important
                            resource"
                        )
                    ),
                    false							# Locked
                ),
                "icon_url" => Tuple(				# Resource URL
                    Enum<1u8>(
                        Enum<13u8>(
                            "https://i.imgur.com/A2itmif.jpeg"
                        )
                    ),
                    false							# Locked
                ),
                "symbol" => Tuple(					# Resource SYMBOL
                    Enum<1u8>(
                        Enum<0u8>("VIP")
                    ),
                    false							# Locked
                )
            ),
            Map<String, Enum>()						# Metadata roles
        )
        Enum<0u8>()									# None | No Address Reservation
    ;
    CALL_METHOD
        Address("account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk")
        "try_deposit_batch_or_abort"
        Expression("ENTIRE_WORKTOP")
        Enum<0u8>()
    ;
        "#;
        let encoded_receipt_hex = "5c22000121062108a000743ba40b000000000000000000000000000000000000000900e1f5050900093d00a000743ba40b000000000000000000000000000000000000000980f0fa02a0aaaa829007e54be700000000000000000000000000000000a080cdc975bc56000000000000000000000000000000000000a080cdc975bc560000000000000000000000000000000000002102080000a0000000a1edccce1bc2d30000000000000000000000000000210709633830000966672300a000dc5d44935b310200000000000000000000000000000000a000385c37d8279c0100000000000000000000000000000000a0000000000000000000000000000000000000000000000000a080dd7d83023a840300000000000000000000000000000000a00000000000000000000000000000000000000000000000002201012102230c091f0b4166746572496e766f6b65060200000e416c6c6f636174654e6f64654964940700000c4265666f7265496e766f6b65540a00000d436c6f73655375627374617465d26900000a4372656174654e6f6465c44200000844726f704e6f64658068000009456d69744576656e7400090000174d61726b537562737461746541735472616e7369656e74370000000a4d6f76654d6f64756c65c81400002b4f70656e53756273746174653a3a476c6f62616c46756e6769626c655265736f757263654d616e61676572952b00001b4f70656e53756273746174653a3a476c6f62616c5061636b61676575f41e00294f70656e53756273746174653a3a476c6f62616c5669727475616c456432353531394163636f756e74d4340600234f70656e53756273746174653a3a496e7465726e616c46756e6769626c655661756c7476180000264f70656e53756273746174653a3a496e7465726e616c47656e65726963436f6d706f6e656e74c6b100000750696e4e6f6465cc0000000a51756572794163746f72e80300000c526561645375627374617465aefb01001c52756e4e6174697665436f64653a3a576f726b746f705f647261696ed82b00001b52756e4e6174697665436f64653a3a576f726b746f705f64726f70fe4500001a52756e4e6174697665436f64653a3a576f726b746f705f707574697100001552756e4e6174697665436f64653a3a637265617465106000003952756e4e6174697665436f64653a3a6372656174655f656d7074795f7661756c745f46756e6769626c655265736f757263654d616e61676572f28a00001f52756e4e6174697665436f64653a3a6372656174655f776974685f646174614f6b00004d52756e4e6174697665436f64653a3a6372656174655f776974685f696e697469616c5f737570706c795f616e645f616464726573735f46756e6769626c655265736f757263654d616e616765723aa201002852756e4e6174697665436f64653a3a6765745f616d6f756e745f46756e6769626c654275636b6574105600002052756e4e6174697665436f64653a3a7075745f46756e6769626c655661756c74ea5f00002952756e4e6174697665436f64653a3a7472795f6465706f7369745f62617463685f6f725f61626f7274a9d901000b5365745375627374617465b90100001156616c696461746554785061796c6f6164a84800001256657269667954785369676e617475726573000000000d57726974655375627374617465161c0000230c09050c436f6d6d69744576656e7473464e00000a436f6d6d69744c6f67730000000031436f6d6d69745374617465557064617465733a3a476c6f62616c46756e6769626c655265736f757263654d616e616765720f851e002f436f6d6d69745374617465557064617465733a3a476c6f62616c5669727475616c456432353531394163636f756e74ab86010029436f6d6d69745374617465557064617465733a3a496e7465726e616c46756e6769626c655661756c74660d0300220001210921012320220b071e860c6318c6318c6c4e1b40cc6318c6318cf7bca52eb54a6a86318c6318c60001230722014000012322220100010702000120074a5c220001210222000121022307a0010260fccd7f5b6fd401000000000000000000000000000000009058619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed7220000071e82cc6318c6318c659963ed8c6318c6318cf7e8f5ae8f4a96a6318c6318c6000123072202400001232222010001070000012007265c220001210222000121050a75a40000000000000768074107ff0a64000000000000002200006800012322220101012007245c200720adf41e2ad5c698c1cccd3a1a6b7d9bbf24c8cde0b7b63fbb7fe497f6785025d100012007125c2200012102220101220001220000220000071e0d906318c6318c659a6130cc6318c6318cf7a8ba5295eabf46318c6318c6000123072206440001232222004200012322220041000123222200010001232222004500012322220046000123222200071e0d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c600012307220744000123222200000001232222004200012322220041000123222200010001232222004500012322220046000123222200071e5dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d22730001230722050200012322220401012007075c0c046e616d65000120071e5c22000121022201012200012200010c0a4d795265736f75726365220000010120070e5c0c0b6465736372697074696f6e000120073c5c22000121022201012200012200010c2841207665727920696e6e6f76617469766520616e6420696d706f7274616e74207265736f7572636522000001012007095c0c0673796d626f6c00012007175c22000121022201012200012200010c03564950220000010120070b5c0c0869636f6e5f75726c00012007345c2200012102220101220001220d010c2068747470733a2f2f692e696d6775722e636f6d2f413269746d69662e6a706567220000050001232222010001070000012007145c220001210222000121022201002200002201000600012322220c01012007115c21022200000c096465706f7369746f7200012007125c220001210222010122000122000022000001012007165c21022200000c0e6275726e65725f7570646174657200012007125c2200012102220101220001220100220000010120070f5c21022200000c07667265657a657200012007125c220001210222010122000122010022000001012007175c21022200000c0f667265657a65725f7570646174657200012007125c2200012102220101220001220100220000010120070e5c21022200000c066275726e657200012007125c220001210222010122000122010022000001012007105c21022200000c08726563616c6c657200012007125c220001210222010122000122010022000001012007125c21022200000c0a7769746864726177657200012007125c220001210222010122000122000022000001012007185c21022200000c10726563616c6c65725f7570646174657200012007125c220001210222010122000122010022000001012007195c21022200000c116465706f7369746f725f7570646174657200012007125c2200012102220101220001220100220000010120070e5c21022200000c066d696e74657200012007125c2200012102220101220001220100220000010120071a5c21022200000c12776974686472617765725f7570646174657200012007125c220001210222010122000122010022000001012007165c21022200000c0e6d696e7465725f7570646174657200012007125c22000121022201012200012201002200004000012322220200010700000120070e5c2200012102220001070a2201000001070100012007255c2200012102220001a0000080f64ae1c7022d1500000000000000000000000000002201000000012322220100010700000120079a015c220001210221052102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c1746756e6769626c655265736f757263654d616e616765722103090100000009000000000900000000220100200c0112747261636b5f746f74616c5f737570706c7920220022000123222102030003090100000009000000000900000000010003090100000009000000000900000000071e0d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c600012307220744000123222200000001232222004200012322220041000123222200010001232222004500012322220046000123222200071e0d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c600012307220744000123222200000001232222004200012322220041000123222200010001232222004500012322220046000123222200071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de60001230722040000012322220042000123222200400001232222004100012322220101012007205c805dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d2273000120072e5c220001210222010122000190588d6bb5373166fe067911568338975207935eff619c82a597cf09687488220000071e0d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60001230722084400012322220000000123222200430001232222004200012322220041000123222200010001232222004500012322220046000123222200071e588d6bb5373166fe067911568338975207935eff619c82a597cf09687488000123072203000001232222010001070000012007745c220001210221052102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c742103090100000009000000000900000000220001805dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d2273200c0020220022010001000123222200400001232222010001070000012007255c2200012102220001a0000080f64ae1c7022d150000000000000000000000000000220000071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed7000123072201400001232222010001070000012007255c2200012102220001a095adf6dfb7dea8030000000000000000000000000000000022000021052080002080002080015dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d2273208001588d6bb5373166fe067911568338975207935eff619c82a597cf0968748823202102071e588d6bb5373166fe067911568338975207935eff619c82a597cf0968748802805dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d2273220001a0000080f64ae1c7022d150000000000000000000000000000071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed702805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6220001a0c0f89bffb6dea8030000000000000000000000000000000021012320a0002104a060fccd7f5b6fd40100000000000000000000000000000000a060fccd7f5b6fd40100000000000000000000000000000000a0c0f89bffb6dea803000000000000000000000000000000002322a00022000120220200012007415c2102805dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d227390f8c18b48bc9ffc4a2b5a327d50045aee7afde33fe1a366b4f068517e76ef00012007035c210020210602210222010220071e5dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d22732200000c194d696e7446756e6769626c655265736f757263654576656e7420071c5c2101a0000080f64ae1c7022d15000000000000000000000000000002210222010220071e5dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d22732200000c125661756c744372656174696f6e4576656e742007245c210120071e588d6bb5373166fe067911568338975207935eff619c82a597cf0968748802210222010220071e588d6bb5373166fe067911568338975207935eff619c82a597cf096874882200000c0c4465706f7369744576656e7420071c5c2101a0000080f64ae1c7022d15000000000000000000000000000002210222010220071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de62200000c0c4465706f7369744576656e7420073c5c220002805dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d2273a0000080f64ae1c7022d15000000000000000000000000000002210222010220071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed72200000c0c4465706f7369744576656e7420071c5c2101a0c0f89bffb6dea8030000000000000000000000000000000002210222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c194275726e46756e6769626c655265736f757263654576656e7420071c5c2101a0c0f89bffb6dea80300000000000000000000000000000000202100210223202306071e860c6318c6318c6c4e1b40cc6318c6318cf7bca52eb54a6a86318c6318c607230140222201000107020301210122000121012103800d906318c6318c6c4e1b40cc6318c6318cf7bfd5d45f48c686318c6318c6200720d8510877df1d820f4752b3c033baf656f62e0e612731718865d048b9d16300b32201010a0900000000000000071e82cc6318c6318c659963ed8c6318c6318cf7e8f5ae8f4a96a6318c6318c607230240222201000107000301210122000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a03000000000000006822220101012007245c200720adf41e2ad5c698c1cccd3a1a6b7d9bbf24c8cde0b7b63fbb7fe497f6785025d10401210222000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a000000000000000022000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a0100000000000000071e5dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d22730723050222220401012007075c0c046e616d650401210222000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a220001070c22000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a2201010a0000000000000000010120070e5c0c0b6465736372697074696f6e0401210222000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a220001070c22000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a2201010a000000000000000001012007095c0c0673796d626f6c0401210222000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a220001070c22000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a2201010a0000000000000000010120070b5c0c0869636f6e5f75726c0401210222000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a220001070c22000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a2201010a000000000000000005222201000107000301210122000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a00000000000000000622220c01012007115c21022200000c096465706f7369746f720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000001012007165c21022200000c0e6275726e65725f757064617465720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a0400000000000000010120070f5c21022200000c07667265657a65720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000001012007175c21022200000c0f667265657a65725f757064617465720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a0400000000000000010120070e5c21022200000c066275726e65720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000001012007105c21022200000c08726563616c6c65720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000001012007125c21022200000c0a776974686472617765720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000001012007185c21022200000c10726563616c6c65725f757064617465720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000001012007195c21022200000c116465706f7369746f725f757064617465720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a0400000000000000010120070e5c21022200000c066d696e7465720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a0400000000000000010120071a5c21022200000c12776974686472617765725f757064617465720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000001012007165c21022200000c0e6d696e7465725f757064617465720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000040222202000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a0000000000000000000107010301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a0100000000000000002222010001070000012101220000071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de60723014122220101012007205c805dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d22730401210222000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a220001078522000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a0300000000000000071e588d6bb5373166fe067911568338975207935eff619c82a597cf0968748807230200222201000107000001210122000040222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed707230140222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000232121060222010220071e5dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d22732200000c194d696e7446756e6769626c655265736f757263654576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a39000000000000000222010220071e5dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d22732200000c125661756c744372656174696f6e4576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a37000000000000000222010220071e588d6bb5373166fe067911568338975207935eff619c82a597cf096874882200000c0c4465706f7369744576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a19000000000000000222010220071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de62200000c0c4465706f7369744576656e740121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a28000000000000000222010220071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed72200000c0c4465706f7369744576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a19000000000000000222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c194275726e46756e6769626c655265736f757263654576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a3a0000000000000022010121032021010822000121022102800d906318c6318c659a6130cc6318c6318cf7a8ba5295eabf46318c6318c60c145472616e73616374696f6e50726f636573736f720c0372756e0a00000000000000002201000a01000000000000000a000000000000000021022320220023202200210223202200232022002021040822000121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c1746756e6769626c655265736f757263654d616e616765720c1a6372656174655f776974685f696e697469616c5f737570706c790a01000000000000002201000a02000000000000000a000000000000000021022320220023202200210223202201071ef8c18b48bc9ffc4a2b5a327d50045aee7afde33fe1a366b4f068517e76ef0002805dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d2273a0000080f64ae1c7022d15000000000000000000000000000023202200202101082202000a02000000000000002201000a02000000000000000a000000000000000021022320220023202200210223202201071ef8c18b48bc9ffc4a2b5a327d50045aee7afde33fe1a366b4f068517e76ef0002805dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d2273a0000080f64ae1c7022d150000000000000000000000000000232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c07576f726b746f700c0b576f726b746f705f7075740a010000000000000022000120071ef8acd314daac79c47fcef350ba25c4440dedf9a77ac7fbf82de3e950070f0a02000000000000000a0000000000000000210223202201071ef8c18b48bc9ffc4a2b5a327d50045aee7afde33fe1a366b4f068517e76ef0002805dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d2273a0000080f64ae1c7022d15000000000000000000000000000023202200210223202200232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c07576f726b746f700c0d576f726b746f705f647261696e0a010000000000000022000120071ef8acd314daac79c47fcef350ba25c4440dedf9a77ac7fbf82de3e950070f0a02000000000000000a010000000000000021022320220023202200210223202201071ef8c18b48bc9ffc4a2b5a327d50045aee7afde33fe1a366b4f068517e76ef0002805dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d2273a0000080f64ae1c7022d150000000000000000000000000000232022002021000822010121022102800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60c074163636f756e740c1a7472795f6465706f7369745f62617463685f6f725f61626f72740a010000000000000022000120071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de60a02000000000000000a0100000000000000210223202201071ef8c18b48bc9ffc4a2b5a327d50045aee7afde33fe1a366b4f068517e76ef0002805dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d2273a0000080f64ae1c7022d15000000000000000000000000000023202200210223202200232022002021010822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c740c037075740a020000000000000022000120071e588d6bb5373166fe067911568338975207935eff619c82a597cf096874880a03000000000000000a0100000000000000210223202201071ef8c18b48bc9ffc4a2b5a327d50045aee7afde33fe1a366b4f068517e76ef0002805dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d2273a0000080f64ae1c7022d1500000000000000000000000000002320220021022320220023202200202101082203000a030000000000000022000120071e588d6bb5373166fe067911568338975207935eff619c82a597cf096874880a03000000000000000a0100000000000000210223202201071ef8c18b48bc9ffc4a2b5a327d50045aee7afde33fe1a366b4f068517e76ef0002805dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d2273a0000080f64ae1c7022d1500000000000000000000000000002320220021022320220023202200202100230a2001010000000000000021010420071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de620071e588d6bb5373166fe067911568338975207935eff619c82a597cf09687488805dfd65b0da5c8621b4862c90dbd8e6460bffe777a82a580b4981248d2273a0000080f64ae1c7022d1500000000000000000000000000002102a0000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000220000";

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

        assert_eq!(sut.fee_locks, FeeLocks::new(0, 0));
        assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.15800815".parse::<Decimal>().unwrap(),
                "0.1160115".parse::<Decimal>().unwrap(),
                "0.25339126151".parse::<Decimal>().unwrap(),
                0,
            )
        );
        assert_eq!(sut.newly_created_non_fungibles, Vec::default());
        assert_eq!(sut.presented_proofs, Vec::default());
        assert_eq!(sut.encountered_component_addresses, Vec::default());
        assert_eq!(sut.addresses_of_accounts_requiring_auth, Vec::default());
        assert_eq!(sut.addresses_of_identities_requiring_auth, Vec::default());
        assert_eq!(sut.detailed_classification.len(), 1);

        assert_eq!(
            sut.detailed_classification,
            vec![DetailedManifestClass::General]
        )
    }



    #[test]
    fn execution_summary_create_pool() {
        let instructions_string = r#"
        CALL_FUNCTION
            Address("package_tdx_2_1pkgxxxxxxxxxplxxxxxxxxxxxxx020379220524xxxxxxxxxe4r780")
            "TwoResourcePool"
            "instantiate"
            Enum<1u8>(
                Enum<0u8>()
            )
            Enum<0u8>()
                Tuple(
                Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc"),
                Address("resource_tdx_2_1thw7yclz24h5xjp3086cj8z2ya0d7p9mydk0yh68c28ha02uhzrnyy")
            )
            Enum<0u8>()
        ;
        "#;
        let encoded_receipt_hex = "5c22000121062108a000743ba40b000000000000000000000000000000000000000900e1f5050900093d00a000743ba40b000000000000000000000000000000000000000980f0fa02a0aaaa829007e54be700000000000000000000000000000000a080cdc975bc56000000000000000000000000000000000000a080cdc975bc560000000000000000000000000000000000002102080000a0000000a1edccce1bc2d300000000000000000000000000002107094ea52d0009aa892f00a00058e1bfd962130200000000000000000000000000000000a000088f2c8969290200000000000000000000000000000000a0000000000000000000000000000000000000000000000000a0006f9b61cbe7900400000000000000000000000000000000a00000000000000000000000000000000000000000000000002201012102230c091d0b4166746572496e766f6b659e0200000e416c6c6f636174654e6f64654964fd0a00000c4265666f7265496e766f6b65860d00000d436c6f736553756273746174654f6800000a4372656174654e6f6465fe6000000844726f704e6f64656289000009456d69744576656e74b4060000174d61726b537562737461746541735472616e7369656e74a50000000a4d6f76654d6f64756c65b81f00002b4f70656e53756273746174653a3a476c6f62616c46756e6769626c655265736f757263654d616e61676572214a02002e4f70656e53756273746174653a3a476c6f62616c4e6f6e46756e6769626c655265736f757263654d616e61676572f4ab00001b4f70656e53756273746174653a3a476c6f62616c5061636b616765d0581f00234f70656e53756273746174653a3a476c6f62616c54776f5265736f75726365506f6f6ccd080000234f70656e53756273746174653a3a496e7465726e616c46756e6769626c655661756c74cf120000264f70656e53756273746174653a3a496e7465726e616c47656e65726963436f6d706f6e656e741e6b00000750696e4e6f6465080100000a51756572794163746f72dc0500000c52656164537562737461746505ca01001b52756e4e6174697665436f64653a3a576f726b746f705f64726f70fe4500001552756e4e6174697665436f64653a3a637265617465332901002d52756e4e6174697665436f64653a3a6372656174655f46756e6769626c655265736f757263654d616e6167657265e800003952756e4e6174697665436f64653a3a6372656174655f656d7074795f7661756c745f46756e6769626c655265736f757263654d616e61676572d6a001001f52756e4e6174697665436f64653a3a6372656174655f776974685f646174619ed600003852756e4e6174697665436f64653a3a6765745f7265736f757263655f747970655f46756e6769626c655265736f757263654d616e6167657240a000002c52756e4e6174697665436f64653a3a696e7374616e74696174655f74776f5f7265736f757263655f706f6f6cffb701000b5365745375627374617465740300001156616c696461746554785061796c6f6164002300001256657269667954785369676e617475726573000000000d57726974655375627374617465281c0000230c09050c436f6d6d69744576656e7473b33a00000a436f6d6d69744c6f67730000000031436f6d6d69745374617465557064617465733a3a476c6f62616c46756e6769626c655265736f757263654d616e616765723cf1190029436f6d6d69745374617465557064617465733a3a476c6f62616c54776f5265736f75726365506f6f6c89350c0029436f6d6d69745374617465557064617465733a3a496e7465726e616c46756e6769626c655661756c74322809002200012109210123202211071e860c6318c6318c6c4e1b40cc6318c6318cf7bca52eb54a6a86318c6318c60001230722014000012322220100010702000120074a5c220001210222000121022307a00103c0f382930b6d3302000000000000000000000000000000009058619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed7220000071e82cc6318c6318c659963ed8c6318c6318cf7e8f5ae8f4a96a6318c6318c6000123072202400001232222010001070000012007265c220001210222000121050a75a40000000000000768074107ff0a64000000000000002200006800012322220101012007245c200720ce169ec268a0d05077361681fcbcdf28da31024b006545d785a55926b07c255e00012007125c2200012102220101220001220000220000071e5ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5c0001230722020000012322220040000123222200071e0d906318c6318c659a6130cc6318c6318cf7a8ba5295eabf46318c6318c6000123072206440001232222004200012322220041000123222200010001232222004500012322220046000123222200071e0d906318c6318c60fcc6318c6318c6318cf53e3e2a94fa2aa6318c6318c60001230722084400012322220000000123222200430001232222004200012322220041000123222200010001232222004500012322220046000123222200071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c60001230722020000012322220040000123222200071e0d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c600012307220744000123222200000001232222004200012322220041000123222200010001232222004500012322220046000123222200071ec532bdebb40b2bf8b4faefa86f5d2a6662c8f03422a49b85722273a8b0150001230722060200012322220301012007145c0c11706f6f6c5f7661756c745f6e756d62657200012007145c2200012102220101220001220201070222010001012007115c0c0e706f6f6c5f7265736f757263657300012007515c22000121022201012200012288012080025da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c65ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5c220100010120070c5c0c09706f6f6c5f756e697400012007315c2200012102220101220001220801805d3d6fbedc64f39bf5896178f80a6833b79b1400ae70063c88a15c78f4c02201000300012322220100010700000120072d5c2200012102220001210190587453a38079270ca174f625449bc34c2fd59a310b03ed66675f7eee5ec3220100050001232222010001070000012007145c220001210222000121022200002200002201000600012322220101012007195c21022200000c11706f6f6c5f6d616e616765725f726f6c6500012007125c2200012102220101220001220000220000400001232222010001070000012007b0015c2200012102220001210220210202805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c690580f357102bbd0439ddee2de32ca2ee51345945fd183bca6079ea005b38f02805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5c9058bd28af20d568763a456e676d1bcf8f4ec05732450208b21c82a69088352101805d3d6fbedc64f39bf5896178f80a6833b79b1400ae70063c88a15c78f4c022010000000123222201000107000001200791015c220001210221052102800d906318c6318c60fcc6318c6318c6318cf53e3e2a94fa2aa6318c6318c60c0f54776f5265736f75726365506f6f6c2103090100000009000000000900000000220100200c0020220022000123222103030003090100000009000000000900000000010003090100000009000000000900000000020003090100000009000000000900000000071e9a4c6318c6318c647f1ff8cc6318c6318cf7aa347bdfaa51e6318c6318c600012307220100000123222200071e5d3d6fbedc64f39bf5896178f80a6833b79b1400ae70063c88a15c78f4c00001230722050200012322220101012007075c0c04706f6f6c00012007315c220001210222010122000122080180c532bdebb40b2bf8b4faefa86f5d2a6662c8f03422a49b85722273a8b015220100050001232222010001070000012007145c220001210222000121022200002200002201000600012322220c01012007115c21022200000c096465706f7369746f7200012007125c220001210222010122000122000022000001012007165c21022200000c0e6275726e65725f7570646174657200012007125c2200012102220101220001220100220000010120070f5c21022200000c07667265657a657200012007125c220001210222010122000122010022000001012007175c21022200000c0f667265657a65725f7570646174657200012007125c2200012102220101220001220100220000010120070e5c21022200000c066275726e6572000120075f5c22000121022201012200012202012200012200012200012102809a4c6318c6318c647f1ff8cc6318c6318cf7aa347bdfaa51e6318c6318c6c0022030eb1c22aef173f2bcb8c0f98e4eec903439447dd106922314273caa1198075722000001012007105c21022200000c08726563616c6c657200012007125c220001210222010122000122010022000001012007125c21022200000c0a7769746864726177657200012007125c220001210222010122000122000022000001012007185c21022200000c10726563616c6c65725f7570646174657200012007125c220001210222010122000122010022000001012007195c21022200000c116465706f7369746f725f7570646174657200012007125c2200012102220101220001220100220000010120070e5c21022200000c066d696e746572000120075f5c22000121022201012200012202012200012200012200012102809a4c6318c6318c647f1ff8cc6318c6318cf7aa347bdfaa51e6318c6318c6c0022030eb1c22aef173f2bcb8c0f98e4eec903439447dd106922314273caa11980757220000010120071a5c21022200000c12776974686472617765725f7570646174657200012007125c220001210222010122000122010022000001012007165c21022200000c0e6d696e7465725f7570646174657200012007125c22000121022201012200012201002200004000012322220200010700000120070e5c220001210222000107122201000001070100012007255c2200012102220001a0000000000000000000000000000000000000000000000000220000000001232222010001070000012007a4015c220001210221052102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c1746756e6769626c655265736f757263654d616e616765722103090100000009000000000900000000220100200c0312747261636b5f746f74616c5f737570706c79046d696e74046275726e20220022000123222102030003090100000009000000000900000000010003090100000009000000000900000000071e0d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c600012307220744000123222200000001232222004200012322220041000123222200010001232222004500012322220046000123222200071e0d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c600012307220744000123222200000001232222004200012322220041000123222200010001232222004500012322220046000123222200071e0d906318c6318c6193bf590c6318c6318cf7c4f52d3d189746318c6318c600012307220744000123222200000001232222004200012322220041000123222200010001232222004500012322220046000123222200071e587453a38079270ca174f625449bc34c2fd59a310b03ed66675f7eee5ec3000123072203000001232222010001070000012007745c220001210221052102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c742103090100000009000000000900000000220001805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6200c0020220022010001000123222200400001232222010001070000012007255c2200012102220001a0000000000000000000000000000000000000000000000000220000071e580f357102bbd0439ddee2de32ca2ee51345945fd183bca6079ea005b38f000123072203000001232222010001070000012007745c220001210221052102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c742103090100000009000000000900000000220001805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6200c0020220022010001000123222200400001232222010001070000012007255c2200012102220001a0000000000000000000000000000000000000000000000000220000071e58bd28af20d568763a456e676d1bcf8f4ec05732450208b21c82a6908835000123072203000001232222010001070000012007745c220001210221052102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c742103090100000009000000000900000000220001805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5c200c0020220022010001000123222200400001232222010001070000012007255c2200012102220001a0000000000000000000000000000000000000000000000000220000071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed7000123072201400001232222010001070000012007255c2200012102220001a02aefcf4617da6604000000000000000000000000000000002200002105208000208001c532bdebb40b2bf8b4faefa86f5d2a6662c8f03422a49b85722273a8b0152080015d3d6fbedc64f39bf5896178f80a6833b79b1400ae70063c88a15c78f4c0208003587453a38079270ca174f625449bc34c2fd59a310b03ed66675f7eee5ec3580f357102bbd0439ddee2de32ca2ee51345945fd183bca6079ea005b38f58bd28af20d568763a456e676d1bcf8f4ec05732450208b21c82a690883523202101071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed702805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6220001a080e7052717da66040000000000000000000000000000000021012320a0002104a0c0f382930b6d330200000000000000000000000000000000a0c0f382930b6d330200000000000000000000000000000000a080e7052717da6604000000000000000000000000000000002322a00022000120220100012007205c80c532bdebb40b2bf8b4faefa86f5d2a6662c8f03422a49b85722273a8b01520210502210222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c125661756c744372656174696f6e4576656e742007245c210120071e587453a38079270ca174f625449bc34c2fd59a310b03ed66675f7eee5ec302210222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c125661756c744372656174696f6e4576656e742007245c210120071e580f357102bbd0439ddee2de32ca2ee51345945fd183bca6079ea005b38f02210222010220071e5ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5c2200000c125661756c744372656174696f6e4576656e742007245c210120071e58bd28af20d568763a456e676d1bcf8f4ec05732450208b21c82a690883502210222010220071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed72200000c0c4465706f7369744576656e7420071c5c2101a080e7052717da66040000000000000000000000000000000002210222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c194275726e46756e6769626c655265736f757263654576656e7420071c5c2101a080e7052717da660400000000000000000000000000000000202100210223202308071e860c6318c6318c6c4e1b40cc6318c6318cf7bca52eb54a6a86318c6318c607230140222201000107020301210122000121012103800d906318c6318c6c4e1b40cc6318c6318cf7bfd5d45f48c686318c6318c6200720d8510877df1d820f4752b3c033baf656f62e0e612731718865d048b9d16300b32201010a0900000000000000071e82cc6318c6318c659963ed8c6318c6318cf7e8f5ae8f4a96a6318c6318c607230240222201000107000301210122000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a03000000000000006822220101012007245c200720ce169ec268a0d05077361681fcbcdf28da31024b006545d785a55926b07c255e0401210222000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a000000000000000022000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a0100000000000000071ec532bdebb40b2bf8b4faefa86f5d2a6662c8f03422a49b85722273a8b0150723060222220301012007145c0c11706f6f6c5f7661756c745f6e756d6265720401210222000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a220001070c22000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a2201010a000000000000000001012007115c0c0e706f6f6c5f7265736f75726365730401210222000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a220001070c22000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a2201010a0000000000000000010120070c5c0c09706f6f6c5f756e69740401210222000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a220001070c22000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a2201010a000000000000000003222201000107000301210122000121012103800d906318c6318c6193bf590c6318c6318cf7c4f52d3d189746318c6318c6200720bbcdf0c14495ebcfb51ade5d02c7ffda015f145e93314136df82d4410bd14d032201010a000000000000000005222201000107000301210122000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a00000000000000000622220101012007195c21022200000c11706f6f6c5f6d616e616765725f726f6c650401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000040222201000107000301210122000121012103800d906318c6318c60fcc6318c6318c6318cf53e3e2a94fa2aa6318c6318c62007200b4ae514b741b2ab034b937f1075da64cbe9ce8b9cd7ff1494fdc14d56cc54332201010a0000000000000000002222010001070000012101220000071e5d3d6fbedc64f39bf5896178f80a6833b79b1400ae70063c88a15c78f4c00723050222220101012007075c0c04706f6f6c0401210222000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a220001070c22000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a2201010a000000000000000005222201000107000301210122000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a00000000000000000622220c01012007115c21022200000c096465706f7369746f720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000001012007165c21022200000c0e6275726e65725f757064617465720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a0400000000000000010120070f5c21022200000c07667265657a65720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000001012007175c21022200000c0f667265657a65725f757064617465720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a0400000000000000010120070e5c21022200000c066275726e65720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000001012007105c21022200000c08726563616c6c65720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000001012007125c21022200000c0a776974686472617765720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000001012007185c21022200000c10726563616c6c65725f757064617465720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000001012007195c21022200000c116465706f7369746f725f757064617465720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a0400000000000000010120070e5c21022200000c066d696e7465720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a0400000000000000010120071a5c21022200000c12776974686472617765725f757064617465720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000001012007165c21022200000c0e6d696e7465725f757064617465720401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000040222202000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a0000000000000000000107010301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a0100000000000000002222010001070000012101220000071e587453a38079270ca174f625449bc34c2fd59a310b03ed66675f7eee5ec307230200222201000107000001210122000040222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000071e580f357102bbd0439ddee2de32ca2ee51345945fd183bca6079ea005b38f07230200222201000107000001210122000040222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000071e58bd28af20d568763a456e676d1bcf8f4ec05732450208b21c82a690883507230200222201000107000001210122000040222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed707230140222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000232121040222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c125661756c744372656174696f6e4576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a37000000000000000222010220071e5ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5c2200000c125661756c744372656174696f6e4576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a37000000000000000222010220071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed72200000c0c4465706f7369744576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a19000000000000000222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c194275726e46756e6769626c655265736f757263654576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a3a000000000000002201012103202100230a20002102a0000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000220000";

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

        assert_eq!(sut.fee_locks, FeeLocks::new(0, 0));
        assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.1495719".parse::<Decimal>().unwrap(),
                "0.1557717".parse::<Decimal>().unwrap(),
                "0.3290176335".parse::<Decimal>().unwrap(),
                0,
            )
        );
        assert_eq!(sut.newly_created_non_fungibles, Vec::default());
        assert_eq!(sut.presented_proofs, Vec::default());
        assert_eq!(sut.encountered_component_addresses, Vec::default());
        assert_eq!(sut.addresses_of_accounts_requiring_auth, Vec::default());
        assert_eq!(sut.addresses_of_identities_requiring_auth, Vec::default());
        assert_eq!(
            sut.detailed_classification,
            vec![DetailedManifestClass::General]
        )
    }

    #[test]
    fn execution_summary_contribute_to_bi_pool() {
        let instructions_string = r#"
    CALL_METHOD
        Address("account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk")
        "withdraw"
        Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
        Decimal("237")
    ;
    TAKE_ALL_FROM_WORKTOP
        Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
        Bucket("bucket1")
    ;
    CALL_METHOD
        Address("account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk")
        "withdraw"
        Address("resource_tdx_2_1thw7yclz24h5xjp3086cj8z2ya0d7p9mydk0yh68c28ha02uhzrnyy")
        Decimal("1337")
    ;
    TAKE_ALL_FROM_WORKTOP
        Address("resource_tdx_2_1thw7yclz24h5xjp3086cj8z2ya0d7p9mydk0yh68c28ha02uhzrnyy")
        Bucket("bucket2")
    ;
    CALL_METHOD
        Address("pool_tdx_2_1ckfjmjswvvf6y635f8l89uunu9cwgnglhqdk8627wrpf8ultdx2vc3")
        "contribute"
        Tuple(
        Bucket("bucket1"),
        Bucket("bucket2")
        )
    ;
    CALL_METHOD
        Address("account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk")
        "try_deposit_batch_or_abort"
        Expression("ENTIRE_WORKTOP")
        Enum<0u8>()
    ;
        "#;
        let encoded_receipt_hex = "5c22000121062108a000743ba40b000000000000000000000000000000000000000900e1f5050900093d00a000743ba40b000000000000000000000000000000000000000980f0fa02a0aaaa829007e54be700000000000000000000000000000000a080cdc975bc56000000000000000000000000000000000000a080cdc975bc560000000000000000000000000000000000002102080000a0000000a1edccce1bc2d300000000000000000000000000002107093bba530009b90c0d00a000bcfba748b7ce0300000000000000000000000000000000a000d4665c21eb970000000000000000000000000000000000a0000000000000000000000000000000000000000000000000a000896916884a7c0200000000000000000000000000000000a00000000000000000000000000000000000000000000000002201012102230c09240b4166746572496e766f6b655c0600000e416c6c6f636174654e6f646549640d1100000c4265666f7265496e766f6b65161200000d436c6f73655375627374617465ce6901000a4372656174654e6f6465e69b00000844726f704e6f6465000f010009456d69744576656e7494190000174d61726b537562737461746541735472616e7369656e74a50000002b4f70656e53756273746174653a3a476c6f62616c46756e6769626c655265736f757263654d616e61676572e90808001b4f70656e53756273746174653a3a476c6f62616c5061636b61676548dd1b00234f70656e53756273746174653a3a476c6f62616c54776f5265736f75726365506f6f6cbf5c0400294f70656e53756273746174653a3a476c6f62616c5669727475616c456432353531394163636f756e74b5a10a00234f70656e53756273746174653a3a496e7465726e616c46756e6769626c655661756c747a6a0500264f70656e53756273746174653a3a496e7465726e616c47656e65726963436f6d706f6e656e7457b602000750696e4e6f6465100200000a51756572794163746f72941100000c526561645375627374617465c82604001c52756e4e6174697665436f64653a3a576f726b746f705f647261696ed82b00001b52756e4e6174697665436f64653a3a576f726b746f705f64726f70fe4500001a52756e4e6174697665436f64653a3a576f726b746f705f7075743b5401001f52756e4e6174697665436f64653a3a576f726b746f705f74616b655f616c6c147200002b52756e4e6174697665436f64653a3a636f6e747269627574655f74776f5f7265736f757263655f706f6f6c350103003952756e4e6174697665436f64653a3a6372656174655f656d7074795f7661756c745f46756e6769626c655265736f757263654d616e61676572f28a00003852756e4e6174697665436f64653a3a64726f705f656d7074795f6275636b65745f46756e6769626c655265736f757263654d616e61676572ea9f00002852756e4e6174697665436f64653a3a6765745f616d6f756e745f46756e6769626c654275636b6574600402002752756e4e6174697665436f64653a3a6765745f616d6f756e745f46756e6769626c655661756c74e67000003752756e4e6174697665436f64653a3a6765745f746f74616c5f737570706c795f46756e6769626c655265736f757263654d616e616765726c4600002b52756e4e6174697665436f64653a3a6d696e745f46756e6769626c655265736f757263654d616e616765723e9900002052756e4e6174697665436f64653a3a7075745f46756e6769626c655661756c74be1f01002152756e4e6174697665436f64653a3a74616b655f46756e6769626c655661756c74b24b01002b52756e4e6174697665436f64653a3a74616b655f616476616e6365645f46756e6769626c654275636b6574d6b500002952756e4e6174697665436f64653a3a7472795f6465706f7369745f62617463685f6f725f61626f7274a9d901001752756e4e6174697665436f64653a3a7769746864726177f6c301001156616c696461746554785061796c6f6164784b00001256657269667954785369676e617475726573000000000d577269746553756273746174656a5e0000230c09050c436f6d6d69744576656e74735bd700000a436f6d6d69744c6f67730000000031436f6d6d69745374617465557064617465733a3a476c6f62616c46756e6769626c655265736f757263654d616e61676572a98601002f436f6d6d69745374617465557064617465733a3a476c6f62616c5669727475616c456432353531394163636f756e74ab86010029436f6d6d69745374617465557064617465733a3a496e7465726e616c46756e6769626c655661756c740a2809002200012109210123202211071e860c6318c6318c6c4e1b40cc6318c6318cf7bca52eb54a6a86318c6318c60001230722014000012322220100010702000120074a5c220001210222000121022307a0010320af63d789c33a04000000000000000000000000000000009058619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed7220000071e82cc6318c6318c659963ed8c6318c6318cf7e8f5ae8f4a96a6318c6318c6000123072202400001232222010001070000012007265c220001210222000121050a75a40000000000000768074107ff0a64000000000000002200006800012322220101012007245c2007208db3f67d3d58ba855f65594cd8c45d2f30f83b1f4bec877b9ddbfe56f0c3ab3c00012007125c2200012102220101220001220000220000071e5ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5c000123072203000001232222000600012322220040000123222200071ec5932dca0e6313a26a3449fe72f393e170e44d1fb81b63e95e70c293f3eb0001230722050000012322220006000123222200030001232222000400012322220040000123222200071e0d906318c6318c659a6130cc6318c6318cf7a8ba5295eabf46318c6318c6000123072206440001232222004200012322220041000123222200010001232222004500012322220046000123222200071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de60001230722060000012322220006000123222200050001232222004100012322220101012007205c805de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d6869000120072e5c22000121022201012200019058e363e1e7273c7660b9522c710f28ad5f811ba1fda5046cb1a2e97c44092200004200012322220040000123222200071e0d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60001230722084400012322220000000123222200430001232222004200012322220041000123222200010001232222004500012322220046000123222200071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6000123072203000001232222000600012322220040000123222200071e58a55175a08e71450554691ccc8b4bb0fc41da7e3a201a9a53d81c441fa500012307220200000123222200400001232222010001070000012007255c2200012102220001a0800ea1b84f878b2e11020000000000000000000000000000220000071e0d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c600012307220744000123222200000001232222004200012322220041000123222200010001232222004500012322220046000123222200071e583126966d2c20d84b200acb0e35b1510efc96202f689c4f11982c7fa75d00012307220200000123222200400001232222010001070000012007255c2200012102220001a000003cbd05b12d88e4140000000000000000000000000000220000071e0d906318c6318c60fcc6318c6318c6318cf53e3e2a94fa2aa6318c6318c60001230722084400012322220000000123222200430001232222004200012322220041000123222200010001232222004500012322220046000123222200071e58dade08212c72f064877db968de9531f4a596f97c48109f4f13a11b04f800012307220200000123222200400001232222010001070000012007255c2200012102220001a00000443945309a7a48000000000000000000000000000000220000071e58cc081196d9c487ca35b1e3fb43eeea8142e443df2792f207c33324785d00012307220200000123222200400001232222010001070000012007255c2200012102220001a0000094f7512409d90c000000000000000000000000000000220000071e5de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d686900012307220300000123222200400001232222010001070100012007255c2200012102220001a09a2483ec8c42f6831e00000000000000000000000000000022000006000123222200071e58e363e1e7273c7660b9522c710f28ad5f811ba1fda5046cb1a2e97c4409000123072203000001232222010001070000012007745c220001210221052102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c742103090100000009000000000900000000220001805de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d6869200c0020220022010001000123222200400001232222010001070000012007255c2200012102220001a09a2483ec8c42f6831e000000000000000000000000000000220000071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed7000123072201400001232222010001070000012007255c2200012102220001a0ea6591ce1387750800000000000000000000000000000000220000210520800020800020800020800158e363e1e7273c7660b9522c710f28ad5f811ba1fda5046cb1a2e97c440923202106071e58a55175a08e71450554691ccc8b4bb0fc41da7e3a201a9a53d81c441fa502805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6220001a000006c08aedbf626f3ffffffffffffffffffffffffffffff071e583126966d2c20d84b200acb0e35b1510efc96202f689c4f11982c7fa75d02805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5c220001a00000bcc6bacf6585b7ffffffffffffffffffffffffffffff071e58dade08212c72f064877db968de9531f4a596f97c48109f4f13a11b04f802805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5c220001a00000443945309a7a48000000000000000000000000000000071e58cc081196d9c487ca35b1e3fb43eeea8142e443df2792f207c33324785d02805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6220001a0000094f7512409d90c000000000000000000000000000000071e58e363e1e7273c7660b9522c710f28ad5f811ba1fda5046cb1a2e97c440902805de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d6869220001a09a2483ec8c42f6831e000000000000000000000000000000071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed702805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6220001a0800c660d797671030000000000000000000000000000000021012320a0002104a04006b3863cbbb80100000000000000000000000000000000a04006b3863cbbb80100000000000000000000000000000000a0800c660d79767103000000000000000000000000000000002322a00022000120220600012007205c90f8285c60db3683aaaba3d2fc7eab0219bf7f60f76e19d6b23854dcd97f00010000012007205c90f8e0a15f2000c8b47806264aed82d7c1ce84879df27409e7550d9c8bab8b010000012007255c210290f84682fee7bbf764e3f0468bb210a40002d45f75f0c6b485e43a39f23d9e22000000012007035c210020210d02210222010220071e58a55175a08e71450554691ccc8b4bb0fc41da7e3a201a9a53d81c441fa52200000c0d57697468647261774576656e7420071c5c2101a0000094f7512409d90c00000000000000000000000000000002210222010220071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de62200000c0d57697468647261774576656e7420073c5c220002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a0000094f7512409d90c00000000000000000000000000000002210222010220071e583126966d2c20d84b200acb0e35b1510efc96202f689c4f11982c7fa75d2200000c0d57697468647261774576656e7420071c5c2101a00000443945309a7a4800000000000000000000000000000002210222010220071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de62200000c0d57697468647261774576656e7420073c5c220002805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5ca00000443945309a7a4800000000000000000000000000000002210222010220071e5de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d68692200000c194d696e7446756e6769626c655265736f757263654576656e7420071c5c2101a09a2483ec8c42f6831e00000000000000000000000000000002210222010220071e58dade08212c72f064877db968de9531f4a596f97c48109f4f13a11b04f82200000c0c4465706f7369744576656e7420071c5c2101a00000443945309a7a4800000000000000000000000000000002210222010220071e58cc081196d9c487ca35b1e3fb43eeea8142e443df2792f207c33324785d2200000c0c4465706f7369744576656e7420071c5c2101a0000094f7512409d90c00000000000000000000000000000002210222010220071ec5932dca0e6313a26a3449fe72f393e170e44d1fb81b63e95e70c293f3eb2200000c11436f6e747269627574696f6e4576656e7420078c015c21022380a0025ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5c0000443945309a7a480000000000000000000000000000005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6000094f7512409d90c000000000000000000000000000000a09a2483ec8c42f6831e00000000000000000000000000000002210222010220071e5de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d68692200000c125661756c744372656174696f6e4576656e742007245c210120071e58e363e1e7273c7660b9522c710f28ad5f811ba1fda5046cb1a2e97c440902210222010220071e58e363e1e7273c7660b9522c710f28ad5f811ba1fda5046cb1a2e97c44092200000c0c4465706f7369744576656e7420071c5c2101a09a2483ec8c42f6831e00000000000000000000000000000002210222010220071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de62200000c0c4465706f7369744576656e7420073c5c220002805de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d6869a09a2483ec8c42f6831e00000000000000000000000000000002210222010220071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed72200000c0c4465706f7369744576656e7420071c5c2101a0800c660d797671030000000000000000000000000000000002210222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c194275726e46756e6769626c655265736f757263654576656e7420071c5c2101a0800c660d797671030000000000000000000000000000000020210021022320230a071e860c6318c6318c6c4e1b40cc6318c6318cf7bca52eb54a6a86318c6318c607230140222201000107020301210122000121012103800d906318c6318c6c4e1b40cc6318c6318cf7bfd5d45f48c686318c6318c6200720d8510877df1d820f4752b3c033baf656f62e0e612731718865d048b9d16300b32201010a0900000000000000071e82cc6318c6318c659963ed8c6318c6318cf7e8f5ae8f4a96a6318c6318c607230240222201000107000301210122000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a03000000000000006822220101012007245c2007208db3f67d3d58ba855f65594cd8c45d2f30f83b1f4bec877b9ddbfe56f0c3ab3c0401210222000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a000000000000000022000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a0100000000000000071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de60723014122220101012007205c805de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d68690401210222000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a220001078522000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a0300000000000000071e58a55175a08e71450554691ccc8b4bb0fc41da7e3a201a9a53d81c441fa507230140222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000071e583126966d2c20d84b200acb0e35b1510efc96202f689c4f11982c7fa75d07230140222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000071e58dade08212c72f064877db968de9531f4a596f97c48109f4f13a11b04f807230140222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000071e58cc081196d9c487ca35b1e3fb43eeea8142e443df2792f207c33324785d07230140222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000071e5de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d686907230140222201000107010301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a0100000000000000071e58e363e1e7273c7660b9522c710f28ad5f811ba1fda5046cb1a2e97c440907230200222201000107000001210122000040222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed707230140222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a00000000000000002321210c0222010220071e58a55175a08e71450554691ccc8b4bb0fc41da7e3a201a9a53d81c441fa52200000c0d57697468647261774576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a18000000000000000222010220071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de62200000c0d57697468647261774576656e740121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a27000000000000000222010220071e583126966d2c20d84b200acb0e35b1510efc96202f689c4f11982c7fa75d2200000c0d57697468647261774576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a18000000000000000222010220071e5de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d68692200000c194d696e7446756e6769626c655265736f757263654576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a39000000000000000222010220071e58dade08212c72f064877db968de9531f4a596f97c48109f4f13a11b04f82200000c0c4465706f7369744576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a19000000000000000222010220071e58cc081196d9c487ca35b1e3fb43eeea8142e443df2792f207c33324785d2200000c0c4465706f7369744576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a19000000000000000222010220071ec5932dca0e6313a26a3449fe72f393e170e44d1fb81b63e95e70c293f3eb2200000c11436f6e747269627574696f6e4576656e740121012103800d906318c6318c60fcc6318c6318c6318cf53e3e2a94fa2aa6318c6318c62007200b4ae514b741b2ab034b937f1075da64cbe9ce8b9cd7ff1494fdc14d56cc54332201010a15000000000000000222010220071e5de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d68692200000c125661756c744372656174696f6e4576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a37000000000000000222010220071e58e363e1e7273c7660b9522c710f28ad5f811ba1fda5046cb1a2e97c44092200000c0c4465706f7369744576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a19000000000000000222010220071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de62200000c0c4465706f7369744576656e740121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a28000000000000000222010220071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed72200000c0c4465706f7369744576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a19000000000000000222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c194275726e46756e6769626c655265736f757263654576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a3a0000000000000022010121032021010822000121022102800d906318c6318c659a6130cc6318c6318cf7a8ba5295eabf46318c6318c60c145472616e73616374696f6e50726f636573736f720c0372756e0a00000000000000002201000a01000000000000000a0000000000000000210223202200232022002102232022002320220020210a0822010121022102800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60c074163636f756e740c0877697468647261770a010000000000000022000120071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de60a02000000000000000a000000000000000021022320220023202200210223202201071ef8285c60db3683aaaba3d2fc7eab0219bf7f60f76e19d6b23854dcd97f000002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a0000094f7512409d90c000000000000000000000000000000232022002021010822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c740c0474616b650a020000000000000022000120071e58a55175a08e71450554691ccc8b4bb0fc41da7e3a201a9a53d81c441fa50a03000000000000000a000000000000000021022320220023202200210223202201071ef8285c60db3683aaaba3d2fc7eab0219bf7f60f76e19d6b23854dcd97f000002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a0000094f7512409d90c00000000000000000000000000000023202200202101082202000a030000000000000022000120071e58a55175a08e71450554691ccc8b4bb0fc41da7e3a201a9a53d81c441fa50a03000000000000000a000000000000000021022320220023202200210223202201071ef8285c60db3683aaaba3d2fc7eab0219bf7f60f76e19d6b23854dcd97f000002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a0000094f7512409d90c000000000000000000000000000000232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c07576f726b746f700c0b576f726b746f705f7075740a010000000000000022000120071ef8666cf5968b0eb4b92a547924e656f1d51224a2517bef3df43db0258b9f0a02000000000000000a0000000000000000210223202201071ef8285c60db3683aaaba3d2fc7eab0219bf7f60f76e19d6b23854dcd97f000002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a0000094f7512409d90c00000000000000000000000000000023202200210223202200232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c07576f726b746f700c10576f726b746f705f74616b655f616c6c0a010000000000000022000120071ef8666cf5968b0eb4b92a547924e656f1d51224a2517bef3df43db0258b9f0a02000000000000000a010000000000000021022320220023202200210223202201071ef8285c60db3683aaaba3d2fc7eab0219bf7f60f76e19d6b23854dcd97f000002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a0000094f7512409d90c000000000000000000000000000000232022002021000822010121022102800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60c074163636f756e740c0877697468647261770a010000000000000022000120071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de60a02000000000000000a020000000000000021022320220023202200210223202201071ef8e0a15f2000c8b47806264aed82d7c1ce84879df27409e7550d9c8bab8b0002805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5ca00000443945309a7a48000000000000000000000000000000232022002021010822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c740c0474616b650a020000000000000022000120071e583126966d2c20d84b200acb0e35b1510efc96202f689c4f11982c7fa75d0a03000000000000000a020000000000000021022320220023202200210223202201071ef8e0a15f2000c8b47806264aed82d7c1ce84879df27409e7550d9c8bab8b0002805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5ca00000443945309a7a4800000000000000000000000000000023202200202101082202000a030000000000000022000120071e583126966d2c20d84b200acb0e35b1510efc96202f689c4f11982c7fa75d0a03000000000000000a020000000000000021022320220023202200210223202201071ef8e0a15f2000c8b47806264aed82d7c1ce84879df27409e7550d9c8bab8b0002805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5ca00000443945309a7a48000000000000000000000000000000232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c07576f726b746f700c0b576f726b746f705f7075740a010000000000000022000120071ef8666cf5968b0eb4b92a547924e656f1d51224a2517bef3df43db0258b9f0a02000000000000000a0200000000000000210223202201071ef8e0a15f2000c8b47806264aed82d7c1ce84879df27409e7550d9c8bab8b0002805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5ca00000443945309a7a4800000000000000000000000000000023202200210223202200232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c07576f726b746f700c10576f726b746f705f74616b655f616c6c0a010000000000000022000120071ef8666cf5968b0eb4b92a547924e656f1d51224a2517bef3df43db0258b9f0a02000000000000000a030000000000000021022320220023202200210223202201071ef8e0a15f2000c8b47806264aed82d7c1ce84879df27409e7550d9c8bab8b0002805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5ca00000443945309a7a48000000000000000000000000000000232022002021000822010121022102800d906318c6318c60fcc6318c6318c6318cf53e3e2a94fa2aa6318c6318c60c0f54776f5265736f75726365506f6f6c0c0a636f6e747269627574650a010000000000000022000120071ec5932dca0e6313a26a3449fe72f393e170e44d1fb81b63e95e70c293f3eb0a02000000000000000a0400000000000000210223202202071ef8285c60db3683aaaba3d2fc7eab0219bf7f60f76e19d6b23854dcd97f000002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a0000094f7512409d90c000000000000000000000000000000071ef8e0a15f2000c8b47806264aed82d7c1ce84879df27409e7550d9c8bab8b0002805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5ca00000443945309a7a4800000000000000000000000000000023202200210223202201071ef84682fee7bbf764e3f0468bb210a40002d45f75f0c6b485e43a39f23d9e0002805de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d6869a09a2483ec8c42f6831e000000000000000000000000000000232022002021070822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0e46756e6769626c654275636b65740c0d74616b655f616476616e6365640a020000000000000022000120071ef8e0a15f2000c8b47806264aed82d7c1ce84879df27409e7550d9c8bab8b0a03000000000000000a040000000000000021022320220023202200210223202201071ef8afe69a29b2c4627fe262d6472d91b6c2d5725f773a4d8b55d94c9911670002805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5ca00000443945309a7a4800000000000000000000000000000023202200202101082202000a030000000000000022000120071ef8e0a15f2000c8b47806264aed82d7c1ce84879df27409e7550d9c8bab8b0a03000000000000000a040000000000000021022320220023202200210223202201071ef8afe69a29b2c4627fe262d6472d91b6c2d5725f773a4d8b55d94c9911670002805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5ca00000443945309a7a48000000000000000000000000000000232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0e46756e6769626c654275636b65740c0d74616b655f616476616e6365640a020000000000000022000120071ef8285c60db3683aaaba3d2fc7eab0219bf7f60f76e19d6b23854dcd97f000a03000000000000000a040000000000000021022320220023202200210223202201071ef8342b4e62ed77378986b4cda068c336ecdb2583796c16af653d1cf8137f0002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a0000094f7512409d90c00000000000000000000000000000023202200202101082202000a030000000000000022000120071ef8285c60db3683aaaba3d2fc7eab0219bf7f60f76e19d6b23854dcd97f000a03000000000000000a040000000000000021022320220023202200210223202201071ef8342b4e62ed77378986b4cda068c336ecdb2583796c16af653d1cf8137f0002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a0000094f7512409d90c000000000000000000000000000000232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c1746756e6769626c655265736f757263654d616e616765720c046d696e740a020000000000000022000120071e5de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d68690a03000000000000000a040000000000000021022320220023202200210223202201071ef84682fee7bbf764e3f0468bb210a40002d45f75f0c6b485e43a39f23d9e0002805de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d6869a09a2483ec8c42f6831e00000000000000000000000000000023202200202101082202000a030000000000000022000120071e5de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d68690a03000000000000000a040000000000000021022320220023202200210223202201071ef84682fee7bbf764e3f0468bb210a40002d45f75f0c6b485e43a39f23d9e0002805de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d6869a09a2483ec8c42f6831e000000000000000000000000000000232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c740c037075740a020000000000000022000120071e58dade08212c72f064877db968de9531f4a596f97c48109f4f13a11b04f80a03000000000000000a0400000000000000210223202201071ef8afe69a29b2c4627fe262d6472d91b6c2d5725f773a4d8b55d94c9911670002805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5ca00000443945309a7a480000000000000000000000000000002320220021022320220023202200202101082203000a030000000000000022000120071e58dade08212c72f064877db968de9531f4a596f97c48109f4f13a11b04f80a03000000000000000a0400000000000000210223202201071ef8afe69a29b2c4627fe262d6472d91b6c2d5725f773a4d8b55d94c9911670002805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5ca00000443945309a7a4800000000000000000000000000000023202200210223202200232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c740c037075740a020000000000000022000120071e58cc081196d9c487ca35b1e3fb43eeea8142e443df2792f207c33324785d0a03000000000000000a0400000000000000210223202201071ef8342b4e62ed77378986b4cda068c336ecdb2583796c16af653d1cf8137f0002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a0000094f7512409d90c0000000000000000000000000000002320220021022320220023202200202101082203000a030000000000000022000120071e58cc081196d9c487ca35b1e3fb43eeea8142e443df2792f207c33324785d0a03000000000000000a0400000000000000210223202201071ef8342b4e62ed77378986b4cda068c336ecdb2583796c16af653d1cf8137f0002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a0000094f7512409d90c00000000000000000000000000000023202200210223202200232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c1746756e6769626c655265736f757263654d616e616765720c1164726f705f656d7074795f6275636b65740a020000000000000022000120071e5ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5c0a03000000000000000a0400000000000000210223202201071ef8e0a15f2000c8b47806264aed82d7c1ce84879df27409e7550d9c8bab8b0002805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5ca00000000000000000000000000000000000000000000000002320220021022320220023202200202101082203000a030000000000000022000120071e5ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5c0a03000000000000000a0400000000000000210223202201071ef8e0a15f2000c8b47806264aed82d7c1ce84879df27409e7550d9c8bab8b0002805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5ca000000000000000000000000000000000000000000000000023202200210223202200232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c1746756e6769626c655265736f757263654d616e616765720c1164726f705f656d7074795f6275636b65740a020000000000000022000120071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c60a03000000000000000a0400000000000000210223202201071ef8285c60db3683aaaba3d2fc7eab0219bf7f60f76e19d6b23854dcd97f000002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a00000000000000000000000000000000000000000000000002320220021022320220023202200202101082203000a030000000000000022000120071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c60a03000000000000000a0400000000000000210223202201071ef8285c60db3683aaaba3d2fc7eab0219bf7f60f76e19d6b23854dcd97f000002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000000000000000000000000000000000000000000000000023202200210223202200232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c07576f726b746f700c0b576f726b746f705f7075740a010000000000000022000120071ef8666cf5968b0eb4b92a547924e656f1d51224a2517bef3df43db0258b9f0a02000000000000000a0400000000000000210223202201071ef84682fee7bbf764e3f0468bb210a40002d45f75f0c6b485e43a39f23d9e0002805de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d6869a09a2483ec8c42f6831e00000000000000000000000000000023202200210223202200232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c07576f726b746f700c0d576f726b746f705f647261696e0a010000000000000022000120071ef8666cf5968b0eb4b92a547924e656f1d51224a2517bef3df43db0258b9f0a02000000000000000a050000000000000021022320220023202200210223202201071ef84682fee7bbf764e3f0468bb210a40002d45f75f0c6b485e43a39f23d9e0002805de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d6869a09a2483ec8c42f6831e000000000000000000000000000000232022002021000822010121022102800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60c074163636f756e740c1a7472795f6465706f7369745f62617463685f6f725f61626f72740a010000000000000022000120071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de60a02000000000000000a0500000000000000210223202201071ef84682fee7bbf764e3f0468bb210a40002d45f75f0c6b485e43a39f23d9e0002805de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d6869a09a2483ec8c42f6831e00000000000000000000000000000023202200210223202200232022002021010822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c740c037075740a020000000000000022000120071e58e363e1e7273c7660b9522c710f28ad5f811ba1fda5046cb1a2e97c44090a03000000000000000a0500000000000000210223202201071ef84682fee7bbf764e3f0468bb210a40002d45f75f0c6b485e43a39f23d9e0002805de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d6869a09a2483ec8c42f6831e0000000000000000000000000000002320220021022320220023202200202101082203000a030000000000000022000120071e58e363e1e7273c7660b9522c710f28ad5f811ba1fda5046cb1a2e97c44090a03000000000000000a0500000000000000210223202201071ef84682fee7bbf764e3f0468bb210a40002d45f75f0c6b485e43a39f23d9e0002805de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d6869a09a2483ec8c42f6831e0000000000000000000000000000002320220021022320220023202200202100230a2004000000000000000021010420071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de620071e58a55175a08e71450554691ccc8b4bb0fc41da7e3a201a9a53d81c441fa5805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000006c08aedbf626f3ffffffffffffffffffffffffffffff020000000000000021010420071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de620071e583126966d2c20d84b200acb0e35b1510efc96202f689c4f11982c7fa75d805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5ca00000bcc6bacf6585b7ffffffffffffffffffffffffffffff040000000000000021020420071ec5932dca0e6313a26a3449fe72f393e170e44d1fb81b63e95e70c293f3eb20071e58dade08212c72f064877db968de9531f4a596f97c48109f4f13a11b04f8805ddde263e2556f43483179f5891c4a275edf04bb236cf25f47c28f7ebd5ca00000443945309a7a480000000000000000000000000000000420071ec5932dca0e6313a26a3449fe72f393e170e44d1fb81b63e95e70c293f3eb20071e58cc081196d9c487ca35b1e3fb43eeea8142e443df2792f207c33324785d805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a0000094f7512409d90c000000000000000000000000000000050000000000000021010420071e51cf94df7259ec2c67de0f475c98a5f194244feca000c2fd613011ee0de620071e58e363e1e7273c7660b9522c710f28ad5f811ba1fda5046cb1a2e97c4409805de77de675721455cc03486c5729857c35511fab2c32bbce5befb47d6869a09a2483ec8c42f6831e0000000000000000000000000000002102a0000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000220000";

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();
        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

         /*
✨✨✨✨✨✨✨
 exeuction summary:

🌱 encounteredEntities ["account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk", "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc", "resource_tdx_2_1thw7yclz24h5xjp3086cj8z2ya0d7p9mydk0yh68c28ha02uhzrnyy", "pool_tdx_2_1ckfjmjswvvf6y635f8l89uunu9cwgnglhqdk8627wrpf8ultdx2vc3"]
🌱 accountsRequiringAuth ["account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk"]
🌱 identitiesRequiringAuth []
▿ EngineToolkit.DetailedManifestClass.poolContribution
  ▿ poolContribution: (2 elements)
    ▿ poolAddresses: 1 element
      ▿ EngineToolkit.Address #0
        ▿ pointer: 0x00000002827cd750
          - pointerValue: 10779154256
    ▿ poolContributions: 1 element
      ▿ EngineToolkit.TrackedPoolContribution
        ▿ poolAddress: EngineToolkit.Address #1
          ▿ pointer: 0x00000002827cc250
            - pointerValue: 10779148880
        ▿ contributedResources: 2 key/value pairs
          ▿ (2 elements)
            - key: "resource_tdx_2_1thw7yclz24h5xjp3086cj8z2ya0d7p9mydk0yh68c28ha02uhzrnyy"
            ▿ value: EngineToolkit.Decimal #2
              ▿ pointer: 0x00000002827cd060
                - pointerValue: 10779152480
          ▿ (2 elements)
            - key: "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc"
            ▿ value: EngineToolkit.Decimal #3
              ▿ pointer: 0x00000002827cd120
                - pointerValue: 10779152672
        ▿ poolUnitsResourceAddress: EngineToolkit.Address #4
          ▿ pointer: 0x00000002827cff10
            - pointerValue: 10779164432
        ▿ poolUnitsAmount: EngineToolkit.Decimal #5
          ▿ pointer: 0x00000002827ce6b0
            - pointerValue: 10779158192
🌱 manifestClass: poolContribution(poolAddresses: [EngineToolkit.Address], poolContributions: [EngineToolkit.TrackedPoolContribution(poolAddress: EngineToolkit.Address, contributedResources: ["resource_tdx_2_1thw7yclz24h5xjp3086cj8z2ya0d7p9mydk0yh68c28ha02uhzrnyy": EngineToolkit.Decimal, "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc": EngineToolkit.Decimal], poolUnitsResourceAddress: EngineToolkit.Address, poolUnitsAmount: EngineToolkit.Decimal)])

✨✨✨✨✨✨✨✨✨

         */

        assert_eq!(sut.fee_locks, FeeLocks::new(0, 0));
        assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.27435815".parse::<Decimal>().unwrap(),
                "0.04276125".parse::<Decimal>().unwrap(),
                "0.17910003354".parse::<Decimal>().unwrap(),
                0,
            )
        );
        let acc_gk: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".parse().unwrap();
        assert_eq!(sut.newly_created_non_fungibles, Vec::default());
        assert_eq!(sut.presented_proofs, Vec::default());
        assert_eq!(sut.encountered_component_addresses, Vec::default());
        assert_eq!(sut.addresses_of_accounts_requiring_auth, vec![acc_gk]);
        assert_eq!(sut.addresses_of_identities_requiring_auth, Vec::default());

        assert_eq!(
            sut.detailed_classification.len(),
            1
        );

        let (pool_addresses, pool_contributions) = sut.detailed_classification[0].clone().into_pool_contribution().unwrap();
        assert_eq!(pool_addresses, vec!["".parse::<>()]);
    }



    #[test]
    fn execution_summary_invalid_receipt() {
        assert_eq!(
            TransactionManifest::sample()
                .execution_summary(BagOfBytes::from_hex("dead").unwrap()),
            Err(CommonError::FailedToDecodeEncodedReceipt)
        );
    }

    #[test]
    fn network_id() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
        assert_eq!(SUT::sample_other().network_id(), NetworkID::Simulator);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn manifest_summary() {
        let manifest = SUT::sample();
        let summary = manifest.summary();
        assert_eq!(summary.addresses_of_accounts_requiring_auth[0].address(), "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease");
    }

    #[test]
    fn resource_addresses_to_refresh() {
        let manifest = SUT::sample();
        let resources = manifest.resource_addresses_to_refresh();
        assert_eq!(resources[0].address(), "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd");
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn samples() {
        assert_eq!(new_transaction_manifest_sample(), SUT::sample());
        assert_eq!(
            new_transaction_manifest_sample_other(),
            SUT::sample_other()
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(
            transaction_manifest_to_string(&SUT::sample()),
            SUT::sample().to_string()
        );
    }

    #[test]
    fn test_new_transaction_manifest_from_instructions_string_and_blobs() {
        let s = new_transaction_manifest_sample().instructions_string();

        assert_eq!(
            new_transaction_manifest_from_instructions_string_and_blobs(
                s.clone(),
                NetworkID::Mainnet,
                Blobs::default()
            )
            .unwrap()
            .instructions_string(),
            s
        );
    }
}
