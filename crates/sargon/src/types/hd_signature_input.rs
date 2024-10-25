use crate::prelude::*;

/// The input used to produce a `HDSignature`. Can be used to see two signatures
/// has the same signer, which would be a bug.
#[derive(Clone, PartialEq, Eq, Hash, derive_more::Debug)]
#[debug(
    "HDSignatureInput {{ payload_id: {:#?}, owned_factor_instance: {:#?} }}",
    payload_id,
    owned_factor_instance
)]
pub struct HDSignatureInput<S: Signable> {
    /// Hash which was signed.
    pub payload_id: <S::Payload as Identifiable>::ID,

    /// The account or identity address of the entity which signed the hash,
    /// with expected public key and with derivation path to derive PrivateKey
    /// with.
    pub owned_factor_instance: OwnedFactorInstance,
}
impl <S: Signable> HDSignatureInput<S> {
    /// Constructs a new `HDSignatureInput`.
    pub fn new(
        payload_id: <S::Payload as Identifiable>::ID,
        owned_factor_instance: OwnedFactorInstance,
    ) -> Self {
        Self {
            payload_id,
            owned_factor_instance,
        }
    }
}

impl HasSampleValues for HDSignatureInput<TransactionIntent> {
    fn sample() -> Self {
        Self::new(
            TransactionIntentHash::sample(),
            OwnedFactorInstance::sample(),
        )
    }
    fn sample_other() -> Self {
        Self::new(
            TransactionIntentHash::sample_other(),
            OwnedFactorInstance::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = HDSignatureInput<TransactionIntent>;

    #[test]
    fn equality_of_samples() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality_of_samples() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            IndexSet::<Sut>::from_iter([
                Sut::sample(),
                Sut::sample_other(),
                Sut::sample(),
                Sut::sample_other()
            ])
            .len(),
            2
        );
    }
}
