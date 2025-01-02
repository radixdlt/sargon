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
pub struct TransactionPreviewRequestOptIns {
    /** This flag controls whether the preview response will include a Radix Engine Toolkit serializable receipt or not. */
    pub radix_engine_toolkit_receipt: bool,
}

impl TransactionPreviewRequestOptIns {
    pub fn new(radix_engine_toolkit_receipt: bool) -> Self {
        Self {
            radix_engine_toolkit_receipt,
        }
    }
}

impl Default for TransactionPreviewRequestOptIns {
    fn default() -> Self {
        Self::new(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionPreviewRequestOptIns;

    #[test]
    fn default_value() {
        let sut = SUT::default();
        assert!(sut.radix_engine_toolkit_receipt);
    }
}
