use crate::prelude::*;

/// The input used to produce a `HDSignature`. Can be used to see two signatures
/// has the same signer, which would be a bug.
#[derive(Clone, PartialEq, Eq, Hash, derive_more::Debug)]
#[debug(
    "HDSignatureInput {{ payload_id: {:#?}, owned_factor_instance: {:#?} }}",
    payload_id,
    owned_factor_instance
)]
pub struct HDSignatureInput<ID: SignableID> {
    /// Hash which was signed.
    pub payload_id: ID,

    /// The account or identity address of the entity which signed the hash,
    /// with expected public key and with derivation path to derive PrivateKey
    /// with.
    pub owned_factor_instance: OwnedFactorInstance,
}
impl<ID: SignableID> HDSignatureInput<ID> {
    /// Constructs a new `HDSignatureInput`.
    pub fn new(
        payload_id: ID,
        owned_factor_instance: OwnedFactorInstance,
    ) -> Self {
        Self {
            payload_id,
            owned_factor_instance,
        }
    }
}

impl<ID: SignableID> HasSampleValues for HDSignatureInput<ID> {
    fn sample() -> Self {
        Self::new(ID::sample(), OwnedFactorInstance::sample())
    }
    fn sample_other() -> Self {
        Self::new(ID::sample_other(), OwnedFactorInstance::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HDSignatureInput<TransactionIntentHash>;

    #[test]
    fn equality_of_samples() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality_of_samples() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            IndexSet::<Sut>::from_iter([
                SUT::sample(),
                SUT::sample_other(),
                SUT::sample(),
                SUT::sample_other()
            ])
            .len(),
            2
        );
    }
}
