pub use crate::prelude::*;
use sargon::Nonce as InternalNonce;

#[derive(
    Clone,
    
    PartialEq,
    Eq,
    Hash,
    
    
     uniffi::Record,
)]
pub struct Nonce {
    value: u32,
}

impl From<InternalNonce> for Nonce {
    fn from(value: InternalNonce) -> Self {
        Self { value: value.0 }
    }
}

impl Into<InternalNonce> for Nonce {
    fn into(self) -> InternalNonce {
        InternalNonce(self.value)
    }
}

#[uniffi::export]
pub fn new_nonce_random() -> Nonce {
    InternalNonce::random().into()
}

#[uniffi::export]
pub fn new_nonce_from_u32(value: u32) -> Nonce {
    InternalNonce::from(value).into()
}

#[uniffi::export]
pub fn new_nonce_sample() -> Nonce {
    InternalNonce::sample().into()
}

#[uniffi::export]
pub fn new_nonce_sample_other() -> Nonce {
    InternalNonce::sample_other().into()
}

#[uniffi::export]
pub fn nonce_get_value(nonce: Nonce) -> u32 {
    u32::from(nonce.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Nonce;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_nonce_sample(),
                new_nonce_sample_other(),
                // duplicates should get removed
                new_nonce_sample(),
                new_nonce_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn random() {
        // with very low probability this will fail.
        assert_ne!(new_nonce_random(), new_nonce_random())
    }

    #[test]
    fn from_u32() {
        let test = |u: u32| assert_eq!(u32::from(new_nonce_from_u32(u)), u);
        test(0);
        test(1);
        test(2);
        test(1337);
    }

    #[test]
    fn to_u32() {
        let test = |u: u32| {
            assert_eq!(
                nonce_get_value(Nonce::from(u32::from(Nonce::from(u)))),
                u
            )
        };
        test(0);
        test(1);
        test(2);
        test(1337);
    }
}
