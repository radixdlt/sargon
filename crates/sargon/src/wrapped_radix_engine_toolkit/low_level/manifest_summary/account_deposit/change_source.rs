use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum ChangeSource {
    InitialYieldFromParent,
    Invocation { instruction_index: u64 },
    NewBucket { instruction_index: u64 },
    Assertion { instruction_index: u64 },
}

impl ChangeSource {
    pub fn invocation(instruction_index: u64) -> Self {
        Self::Invocation { instruction_index }
    }

    pub fn new_bucket(instruction_index: u64) -> Self {
        Self::NewBucket { instruction_index }
    }

    pub fn assertion(instruction_index: u64) -> Self {
        Self::Assertion { instruction_index }
    }
}

impl From<ScryptoChangeSource> for ChangeSource {
    fn from(value: ScryptoChangeSource) -> Self {
        match value {
            ScryptoChangeSource::InitialYieldFromParent => {
                Self::InitialYieldFromParent
            }
            ScryptoChangeSource::Invocation { instruction_index } => {
                Self::Invocation {
                    instruction_index: instruction_index as u64,
                }
            }
            ScryptoChangeSource::NewBucket { instruction_index } => {
                Self::NewBucket {
                    instruction_index: instruction_index as u64,
                }
            }
            ScryptoChangeSource::Assertion { instruction_index } => {
                Self::Assertion {
                    instruction_index: instruction_index as u64,
                }
            }
        }
    }
}

impl HasSampleValues for ChangeSource {
    fn sample() -> Self {
        Self::InitialYieldFromParent
    }

    fn sample_other() -> Self {
        Self::invocation(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ChangeSource;

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
    fn from_scrypto_initial_yield_from_parent() {
        let scrypto = ScryptoChangeSource::InitialYieldFromParent;
        assert_eq!(SUT::from(scrypto), SUT::sample());
    }

    #[test]
    fn from_scrypto_invocation() {
        let scrypto = ScryptoChangeSource::Invocation {
            instruction_index: 1,
        };
        assert_eq!(SUT::from(scrypto), SUT::sample_other());
    }
}
