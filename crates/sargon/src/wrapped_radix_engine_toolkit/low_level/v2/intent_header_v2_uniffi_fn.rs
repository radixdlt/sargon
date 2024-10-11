use crate::prelude::*;

#[uniffi::export]
pub fn new_intent_header_v2_sample() -> IntentHeaderV2 {
    IntentHeaderV2::sample()
}

#[uniffi::export]
pub fn new_intent_header_v2_sample_other() -> IntentHeaderV2 {
    IntentHeaderV2::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IntentHeaderV2;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_intent_header_v2_sample(),
                new_intent_header_v2_sample_other(),
                // duplicates should get removed
                new_intent_header_v2_sample(),
                new_intent_header_v2_sample_other(),
            ])
            .len(),
            2
        );
    }
}
