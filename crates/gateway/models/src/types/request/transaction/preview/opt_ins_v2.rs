use crate::prelude::*;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
)]
pub struct TransactionPreviewRequestOptInsV2 {
    /**
     * This flag controls whether the preview response will include a Core API receipt or not.
     * If not provided, this defaults to false and no core api receipt is provided in the response.
     */
    pub core_api_receipt: bool,

    /**
     * This flag controls whether the preview response will include a Radix Engine Toolkit serializable receipt or not.
     * If not provided, this defaults to false and no toolkit receipt is provided in the response.
     */
    pub radix_engine_toolkit_receipt: bool,

    /**
     * This flag controls whether the preview response will include execution logs.
     *  If not provided, this defaults to false and no logs will be provided in the response.
     */
    pub logs: bool,
}

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
