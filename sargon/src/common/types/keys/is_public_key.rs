pub trait IsPublicKey<S>: Sized {
    fn is_valid(
        &self,
        signature: &S,
        for_hash: &impl radix_engine_common::crypto::IsHash,
    ) -> bool;
}
