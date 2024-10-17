use crate::prelude::*;

impl TransactionManifestV2 {
    pub fn execution_summary(
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
    use radix_engine::transaction::{
        AbortReason, AbortResult, TransactionResult,
    };

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ExecutionSummary;

    #[test]
    fn failure_if_receipt_result_is_abort() {
        let wrong_receipt = ScryptoRuntimeToolkitTransactionReceipt::Abort {
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
