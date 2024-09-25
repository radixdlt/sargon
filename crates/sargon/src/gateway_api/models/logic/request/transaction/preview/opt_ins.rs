use crate::prelude::*;

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
