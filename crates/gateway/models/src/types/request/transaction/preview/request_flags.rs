use core_misc::decl_bool_type;

use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
)]
pub struct TransactionPreviewRequestFlags {
    pub use_free_credit: UseFreeCredit,
    pub assume_all_signature_proofs: AssumeAllSignatureProofs,
    pub skip_epoch_check: SkipEpochCheck,
    pub disable_auth_checks: DisableAuthChecks,
}

decl_bool_type!(UseFreeCredit, true);
decl_bool_type!(AssumeAllSignatureProofs, false);
decl_bool_type!(SkipEpochCheck, false);
decl_bool_type!(DisableAuthChecks, false);

impl TransactionPreviewRequestFlags {
    pub fn new(
        use_free_credit: UseFreeCredit,
        assume_all_signature_proofs: AssumeAllSignatureProofs,
        skip_epoch_check: SkipEpochCheck,
        disable_auth_checks: DisableAuthChecks,
    ) -> Self {
        Self {
            use_free_credit,
            assume_all_signature_proofs,
            skip_epoch_check,
            disable_auth_checks,
        }
    }
}

impl Default for TransactionPreviewRequestFlags {
    fn default() -> Self {
        Self::new(
            UseFreeCredit::default(),
            AssumeAllSignatureProofs::default(),
            SkipEpochCheck::default(),
            DisableAuthChecks::default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionPreviewRequestFlags;

    #[test]
    fn default_is_use_free_credit() {
        assert!(SUT::default().use_free_credit.0);
    }

    #[test]
    fn default_assume_all_signature_proofs() {
        assert!(!SUT::default().assume_all_signature_proofs.0);
    }

    #[test]
    fn default_skip_epoch_check() {
        assert!(!SUT::default().skip_epoch_check.0);
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::default();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "use_free_credit": true,
                "assume_all_signature_proofs": false,
                "skip_epoch_check": false,
                "disable_auth_checks": false
            }
            "#,
        )
    }
}
