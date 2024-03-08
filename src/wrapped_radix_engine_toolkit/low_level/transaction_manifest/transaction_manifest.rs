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

        Ok(Into::<ExecutionSummary>::into((
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
                0,
                "0.16679763507".parse::<Decimal>().unwrap()
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
                0,
                "0.08459091041".parse::<Decimal>().unwrap(),
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
