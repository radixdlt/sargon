use crate::prelude::*;

/// The outcome of the signing process for each factor source as collected by the `SignInteractor`.
#[derive(
    Clone, PartialEq, Eq, enum_as_inner::EnumAsInner, derive_more::Debug,
)]
pub enum FactorOutcome<ID: SignableID> {
    /// The user successfully signed with the factor source, the associated
    /// value `produced_signatures` contains the produced signatures and any relevant metadata.
    #[debug("Signed: {:#?}", produced_signatures)]
    Signed {
        produced_signatures: IndexSet<HDSignature<ID>>,
    },

    /// The factor source got neglected, either due to user explicitly skipping
    /// or due to failure
    #[debug("Neglected")]
    Neglected(NeglectedFactor),
}

impl<ID: SignableID + HasSampleValues> HasSampleValues for FactorOutcome<ID> {
    fn sample() -> Self {
        let signature = HDSignature::<ID>::sample();

        Self::signed(IndexSet::just(signature)).unwrap()
    }

    fn sample_other() -> Self {
        Self::skipped(FactorSourceIDFromHash::sample_other())
    }
}

impl<ID: SignableID> FactorOutcome<ID> {
    pub fn signed(
        produced_signatures: IndexSet<HDSignature<ID>>,
    ) -> Result<Self> {
        if produced_signatures.is_empty() {
            return Err(CommonError::ExpectedNonEmptyCollection);
        }

        let factor_source_id = &produced_signatures
            .first()
            .expect("Should have at least one signature")
            .factor_source_id();

        if produced_signatures
            .iter()
            .any(|s| s.factor_source_id() != *factor_source_id)
        {
            return Err(CommonError::FactorOutcomeSignedFactorSourceIDMismatch);
        }

        Ok(FactorOutcome::Signed {
            produced_signatures,
        })
    }

    pub fn failure(factor: FactorSourceIDFromHash) -> Self {
        FactorOutcome::Neglected(NeglectedFactor::new(
            NeglectFactorReason::Failure,
            factor,
        ))
    }

    pub fn skipped(factor: FactorSourceIDFromHash) -> Self {
        FactorOutcome::Neglected(NeglectedFactor::new(
            NeglectFactorReason::UserExplicitlySkipped,
            factor,
        ))
    }

    pub fn irrelevant(factor: FactorSourceIDFromHash) -> Self {
        FactorOutcome::Neglected(NeglectedFactor::new(
            NeglectFactorReason::Irrelevant,
            factor,
        ))
    }

    pub fn factor_source_id(&self) -> FactorSourceIDFromHash {
        match self {
            FactorOutcome::Signed {
                produced_signatures,
            } => {
                let signature = produced_signatures
                    .first()
                    .expect("Should have at least one signature");

                signature.factor_source_id()
            }
            FactorOutcome::Neglected(neglected_factor) => {
                neglected_factor.factor_source_id()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorOutcome<TransactionIntentHash>;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    pub fn test_signed() {
        let signature = HDSignature::sample();
        let signatures = IndexSet::just(signature.clone());

        let sut = SUT::signed(signatures.clone()).unwrap();

        assert_eq!(sut.as_signed().unwrap().clone(), signatures);
    }

    #[test]
    pub fn test_signed_no_signatures_failure() {
        let result = SUT::signed(IndexSet::new());

        assert_eq!(result, Err(CommonError::ExpectedNonEmptyCollection));
    }

    #[test]
    pub fn test_signed_different_factor_source_id_failure() {
        let result = SUT::signed(
            IndexSet::from([
                HDSignature::fake_sign_by_looking_up_mnemonic_amongst_samples(
                    HDSignatureInput::new(
                        TransactionIntentHash::sample(),
                        OwnedFactorInstance::new(
                            AddressOfAccountOrPersona::sample(),
                            HierarchicalDeterministicFactorInstance::sample_mainnet_tx_account(
                                Hardened::from_local_key_space_unsecurified(0u32).unwrap(),
                                FactorSourceIDFromHash::sample_device(),
                            ),
                        )
                    )
                ),
                HDSignature::fake_sign_by_looking_up_mnemonic_amongst_samples(
                    HDSignatureInput::new(
                        TransactionIntentHash::sample(),
                        OwnedFactorInstance::new(
                            AddressOfAccountOrPersona::sample(),
                            HierarchicalDeterministicFactorInstance::sample_mainnet_tx_account(
                                Hardened::from_local_key_space_unsecurified(1u32).unwrap(),
                                FactorSourceIDFromHash::sample_device_other(),
                            ),
                        )
                    )
                )
            ])
        );

        assert_eq!(
            result,
            Err(CommonError::FactorOutcomeSignedFactorSourceIDMismatch)
        );
    }

    #[test]
    pub fn test_failure() {
        let factor_source_id = FactorSourceIDFromHash::sample();

        let sut = SUT::failure(factor_source_id);

        assert_eq!(sut.as_neglected().unwrap().content, factor_source_id);
        assert_eq!(
            sut.as_neglected().unwrap().reason,
            NeglectFactorReason::Failure
        );
    }

    #[test]
    pub fn test_skipped() {
        let factor_source_id = FactorSourceIDFromHash::sample();

        let sut = SUT::skipped(factor_source_id);

        assert_eq!(sut.as_neglected().unwrap().content, factor_source_id);
        assert_eq!(
            sut.as_neglected().unwrap().reason,
            NeglectFactorReason::UserExplicitlySkipped
        );
    }

    #[test]
    pub fn test_irrelevant() {
        let factor_source_id = FactorSourceIDFromHash::sample();

        let sut = SUT::irrelevant(factor_source_id);

        assert_eq!(sut.as_neglected().unwrap().content, factor_source_id);
        assert_eq!(
            sut.as_neglected().unwrap().reason,
            NeglectFactorReason::Irrelevant
        );
    }
}
