use crate::prelude::*;

impl DynamicallyAnalyzableManifest for ScryptoTransactionManifestV2 {
    fn ret_dynamically_analyze(
        &self,
        receipt: ScryptoRuntimeToolkitTransactionReceipt,
    ) -> Result<RetDynamicAnalysis, RetManifestAnalysisError> {
        RET_dynamically_analyze_v2(self, receipt.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use radix_engine_toolkit_common::receipt::SerializableToolkitTransactionReceipt;

    #[test]
    fn failure_if_receipt_result_is_abort() {
        let wrong_receipt = SerializableToolkitTransactionReceipt::Abort {
            reason: "whatever".to_owned(),
        };

        let manifest = ScryptoTransactionManifestV2Builder::new_v2()
            .assert_worktop_is_empty()
            .drop_all_proofs()
            .build();

        assert_eq!(
            manifest.execution_summary(wrong_receipt, NetworkID::Mainnet),
            Err(CommonError::ExecutionSummaryFail {
                underlying: "NotACommitSuccessReceipt".to_owned()
            })
        );
    }
}
