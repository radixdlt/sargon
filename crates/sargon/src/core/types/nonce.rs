pub use crate::prelude::*;

/// A random number generated part of a transaction header,
/// ensuring every transaction os unique even though its
/// transaction manifest might be equal. This nonce is
/// generated by wallets for incoming transactions.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[display("{}", self.0)]
#[debug("{}", self.0)]
pub struct Nonce(pub u32)

impl From<u32> for Nonce {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Nonce> for u32 {
    fn from(value: Nonce) -> Self {
        value.0
    }
}

impl Nonce {
        pub fn random() -> Self {
            let value = u32::from_be_bytes(
                generate_bytes::<4>()
                    .as_slice()
                    .try_into()
                    .expect("It is 4 bytes."),
            );
    
            Self(value)
        }
}

impl HasSampleValues for Nonce {
    fn sample() -> Self {
        Self(123456789)
    }

    fn sample_other() -> Self {
        Self(987654321)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Nonce;

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
    fn from_u32() {
        let test = |u: u32| assert_eq!(u32::from(Nonce::from(u)), u);
        test(0);
        test(1);
        test(2);
        test(1337);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::sample()), "123456789");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", SUT::sample()), "123456789");
    }

    #[test]
    fn to_u32() {
        let test = |u: u32| {
            assert_eq!(u32::from(Nonce::from(u32::from(Nonce::from(u)))), u)
        };
        test(0);
        test(1);
        test(2);
        test(1337);
    }

    #[test]
    fn generate_new() {
        let mut set: HashSet<SUT> = HashSet::new();
        let n = 100;
        for _ in 0..n {
            let sut = SUT::random();
            set.insert(sut);
        }
        assert_eq!(set.len(), n); // with a low probability this might fail yes.
    }
}
