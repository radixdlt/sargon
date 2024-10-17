use crate::prelude::*;

impl TransactionManifestV2 {
    /// Creates the `ExecutionSummary` based on the `engine_toolkit_receipt`.
    ///
    /// Such value should be obtained from the Gateway `/transaction/preview` endpoint, under the `radix_engine_toolkit_receipt` field.
    pub fn execution_summary(
        &self,
        engine_toolkit_receipt: ScryptoSerializableToolkitTransactionReceipt,
    ) -> Result<ExecutionSummary> {
        let network_definition = self.network_id().network_definition();
        let runtime_receipt = engine_toolkit_receipt
            .into_runtime_receipt(&ScryptoAddressBech32Decoder::new(
                &network_definition,
            ))
            .ok()
            .ok_or(CommonError::FailedToDecodeEngineToolkitReceipt)?;

        self.execution_summary_with_receipt(runtime_receipt)
    }

    pub fn execution_summary_with_receipt(
        &self,
        receipt: ScryptoRuntimeToolkitTransactionReceipt,
    ) -> Result<ExecutionSummary> {
        let ret_dynamic_analysis =
            RET_dynamically_analyze_v2(&self.scrypto_manifest(), &receipt)
                .map_err(|e| {
                    error!(
                        "Failed to get execution summary from RET, error: {:?}",
                        e
                    );
                    CommonError::ExecutionSummaryFail {
                        underlying: format!("{:?}", e),
                    }
                })?;

        Ok(ExecutionSummary::from((
            ret_dynamic_analysis,
            self.network_id(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use radix_engine::transaction::{
        AbortReason, AbortResult, TransactionResult,
    };
    use radix_engine_toolkit_common::receipt::SerializableToolkitTransactionReceipt;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ExecutionSummary;

    #[test]
    fn failure_if_receipt_result_is_abort() {
        let wrong_receipt = SerializableToolkitTransactionReceipt::Abort {
            reason: "whatever".to_owned(),
        };

        assert_eq!(
            TransactionManifestV2::sample().execution_summary(wrong_receipt),
            Err(CommonError::ExecutionSummaryFail {
                underlying: "InvalidReceipt".to_owned()
            })
        );
    }
}
