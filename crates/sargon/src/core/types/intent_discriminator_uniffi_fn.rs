use crate::prelude::*;

#[uniffi::export]
pub fn new_intent_discriminator_random() -> IntentDiscriminator {
    IntentDiscriminator::random()
}

#[uniffi::export]
pub fn new_intent_discriminator_from_u64(value: u64) -> IntentDiscriminator {
    IntentDiscriminator::from(value)
}

#[uniffi::export]
pub fn new_intent_discriminator_sample() -> IntentDiscriminator {
    IntentDiscriminator::sample()
}

#[uniffi::export]
pub fn new_intent_discriminator_sample_other() -> IntentDiscriminator {
    IntentDiscriminator::sample_other()
}

#[uniffi::export]
pub fn intent_discriminator_get_value(
    intent_discriminator: IntentDiscriminator,
) -> u64 {
    u64::from(intent_discriminator)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IntentDiscriminator;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_intent_discriminator_sample(),
                new_intent_discriminator_sample_other(),
                // duplicates should get removed
                new_intent_discriminator_sample(),
                new_intent_discriminator_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn random() {
        // with very low probability this will fail.
        assert_ne!(
            new_intent_discriminator_random(),
            new_intent_discriminator_random()
        )
    }

    #[test]
    fn from_u64() {
        let test = |u: u64| {
            assert_eq!(u64::from(new_intent_discriminator_from_u64(u)), u)
        };
        test(0);
        test(1);
        test(2);
        test(1337);
    }

    #[test]
    fn to_u64() {
        let test = |u: u64| {
            assert_eq!(
                intent_discriminator_get_value(IntentDiscriminator::from(
                    u64::from(IntentDiscriminator::from(u))
                )),
                u
            )
        };
        test(0);
        test(1);
        test(2);
        test(1337);
    }
}
