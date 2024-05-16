pub trait IsPublicKey<S>: Sized {
    fn is_valid_signature_for_hash(
        &self,
        signature: &S,
        hash: &impl radix_common::crypto::IsHash,
    ) -> bool;
}
