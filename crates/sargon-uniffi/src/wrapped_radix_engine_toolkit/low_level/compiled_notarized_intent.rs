use crate::prelude::*;
use sargon::CompiledNotarizedIntent as InternalCompiledNotarizedIntent;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
     uniffi::Record,
)]
pub struct CompiledNotarizedIntent {
    secret_magic: BagOfBytes,
}

impl From<InternalCompiledNotarizedIntent> for CompiledNotarizedIntent {
    fn from(value: InternalCompiledNotarizedIntent) -> Self {
        Self {
            secret_magic: value.secret_magic.into(),
        }
    }
}

impl Into<InternalCompiledNotarizedIntent> for CompiledNotarizedIntent {
    fn into(self) -> InternalCompiledNotarizedIntent {
        InternalCompiledNotarizedIntent(self.secret_magic.into())
    }
}

#[uniffi::export]
pub fn new_compiled_notarized_intent_sample() -> CompiledNotarizedIntent {
    InternalCompiledNotarizedIntent::sample().into()
}

#[uniffi::export]
pub fn new_compiled_notarized_intent_sample_other() -> CompiledNotarizedIntent {
    InternalCompiledNotarizedIntent::sample_other().into()
}

#[uniffi::export]
pub fn compiled_notarized_intent_get_bytes(
    compiled_notarized_intent: &CompiledNotarizedIntent,
) -> BagOfBytes {
    compiled_notarized_intent.into::<InternalCompiledNotarizedIntent>().bytes().into()
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
        assert_eq!(compiled_notarized_intent_get_bytes(&sut), sut.bytes());
    }
}
