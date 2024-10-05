pub use crate::prelude::*;

// Generate the FfiConverter needed by UniFFI for newtype `Nonce`.
uniffi::custom_newtype!(NonceSecretMagic, u32);

#[derive(
    Clone, Copy, PartialEq, Eq, Hash, derive_more::Display, derive_more::Debug,
)]
pub struct NonceSecretMagic(pub u32);

#[uniffi::export]
pub fn new_nonce_random() -> Nonce {
    Nonce::random()
}

#[uniffi::export]
pub fn new_nonce_from_u32(value: u32) -> Nonce {
    Nonce::from(value)
}

#[uniffi::export]
pub fn new_nonce_sample() -> Nonce {
    Nonce::sample()
}

#[uniffi::export]
pub fn new_nonce_sample_other() -> Nonce {
    Nonce::sample_other()
}

#[uniffi::export]
pub fn nonce_get_value(nonce: Nonce) -> u32 {
    u32::from(nonce)
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
