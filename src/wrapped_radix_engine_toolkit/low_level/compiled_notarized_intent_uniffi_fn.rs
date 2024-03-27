use crate::prelude::*;

#[uniffi::export]
pub fn new_compiled_notarized_intent_sample() -> CompiledNotarizedIntent {
    CompiledNotarizedIntent::sample()
}

#[uniffi::export]
pub fn new_compiled_notarized_intent_sample_other() -> CompiledNotarizedIntent {
    CompiledNotarizedIntent::sample_other()
}

#[uniffi::export]
pub fn compiled_notarized_intent_get_bytes(
    compiled_notarized_intent: &CompiledNotarizedIntent,
) -> BagOfBytes {
    compiled_notarized_intent.secret_magic.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = CompiledNotarizedIntent;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_compiled_notarized_intent_sample(),
                new_compiled_notarized_intent_sample_other(),
                // duplicates should get removed
                new_compiled_notarized_intent_sample(),
                new_compiled_notarized_intent_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn get_bytes() {
        let sut = SUT::sample();
        assert_eq!(compiled_notarized_intent_get_bytes(&sut), sut.secret_magic);
    }
}
