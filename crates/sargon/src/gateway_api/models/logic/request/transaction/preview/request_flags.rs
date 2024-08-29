use crate::prelude::*;

impl TransactionPreviewRequestFlags {
    pub fn new(
        use_free_credit: bool,
        assume_all_signature_proofs: bool,
        skip_epoch_check: bool,
    ) -> Self {
        Self {
            use_free_credit,
            assume_all_signature_proofs,
            skip_epoch_check,
        }
    }
}

impl Default for TransactionPreviewRequestFlags {
    fn default() -> Self {
        Self::new(true, false, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionPreviewRequestFlags;

    #[test]
    fn default_value() {
        let sut = SUT::default();
        assert!(sut.use_free_credit);
        assert!(!sut.assume_all_signature_proofs);
        assert!(!sut.skip_epoch_check);
    }
}
