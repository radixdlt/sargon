use crate::prelude::*;

impl TransactionPreviewRequestOptInsV2 {
    pub fn new(
        core_api_receipt: bool,
        radix_engine_toolkit_receipt: bool,
        logs: bool,
    ) -> Self {
        Self {
            core_api_receipt,
            radix_engine_toolkit_receipt,
            logs,
        }
    }
}

impl Default for TransactionPreviewRequestOptInsV2 {
    fn default() -> Self {
        Self::new(true, true, false)
    }
}

impl HasSampleValues for TransactionPreviewRequestOptInsV2 {
    fn sample() -> Self {
        TransactionPreviewRequestOptInsV2::new(false, false, false)
    }

    fn sample_other() -> Self {
        TransactionPreviewRequestOptInsV2::new(true, true, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionPreviewRequestOptInsV2;

    #[test]
    fn default_value() {
        let sut = SUT::default();
        assert!(sut.core_api_receipt);
        assert!(sut.radix_engine_toolkit_receipt);
        assert!(!sut.logs);
    }
}
