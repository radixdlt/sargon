use crate::prelude::*;

impl StaticallyAnalyzableManifest for TransactionManifestV2 {
    fn network_id(&self) -> NetworkID {
        self.network_id()
    }

    fn summary(&self) -> Result<ManifestSummary> {
        let summary = RET_statically_analyze_v2(&self.scrypto_manifest())
            .map_err(map_static_analysis_error)?;
        Ok(ManifestSummary::from((summary, self.network_id())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use radix_engine_toolkit_common::receipt::SerializableToolkitTransactionReceipt;
    use radix_rust::hashmap;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifestV2;

    #[test]
    fn failure_if_receipt_result_is_abort() {
        let wrong_receipt = SerializableToolkitTransactionReceipt::Abort {
            reason: "whatever".to_owned(),
        };

        let scrypto_manifest = ScryptoTransactionManifestV2Builder::new_v2()
            .assert_worktop_is_empty()
            .drop_all_proofs()
            .build();

        let manifest =
            SUT::try_from((scrypto_manifest, NetworkID::Mainnet)).unwrap();

        assert_eq!(
            manifest.execution_summary(wrong_receipt),
            Err(CommonError::ExecutionSummaryFail {
                underlying: "NotACommitSuccessReceipt".to_owned()
            })
        );
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
                [ManifestClassification::Transfer, ManifestClassification::General],
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
                [ManifestClassification::Transfer, ManifestClassification::General],
            )
        );
    }
}
