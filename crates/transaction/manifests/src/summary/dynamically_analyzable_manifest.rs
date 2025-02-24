use crate::prelude::*;

/// Describes a manifest that can be dynamically analyzed
pub trait DynamicallyAnalyzableManifest: StaticallyAnalyzableManifest {
    /// Perform the ret analysis
    fn ret_dynamically_analyze(
        &self,
        receipt: ScryptoRuntimeToolkitTransactionReceipt,
    ) -> Result<RetDynamicAnalysis, RetManifestAnalysisError>;

    /// Get the execution summary on an given network from a given transaction receipt
    fn execution_summary(
        &self,
        engine_toolkit_receipt: ScryptoSerializableToolkitTransactionReceipt,
    ) -> Result<ExecutionSummary> {
        let receipt = engine_toolkit_receipt
            .into_runtime_receipt(&ScryptoAddressBech32Decoder::new(
                &self.network_id().network_definition(),
            ))
            .map_err(|e| {
                error!("Failed to decode engine toolkit receipt  {:?}", e);
                CommonError::FailedToDecodeEngineToolkitReceipt
            })?;

        let ret_dynamic_analysis =
            self.ret_dynamically_analyze(receipt).map_err(|e| {
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

impl DynamicallyAnalyzableManifest for TransactionManifest {
    fn ret_dynamically_analyze(
        &self,
        receipt: ScryptoRuntimeToolkitTransactionReceipt,
    ) -> Result<RetDynamicAnalysis, RetManifestAnalysisError> {
        RET_dynamically_analyze(&self.scrypto_manifest(), receipt)
    }
}

impl DynamicallyAnalyzableManifest for TransactionManifestV2 {
    fn ret_dynamically_analyze(
        &self,
        receipt: ScryptoRuntimeToolkitTransactionReceipt,
    ) -> Result<RetDynamicAnalysis, RetManifestAnalysisError> {
        RET_dynamically_analyze_v2(&self.scrypto_manifest(), receipt.clone())
    }
}
