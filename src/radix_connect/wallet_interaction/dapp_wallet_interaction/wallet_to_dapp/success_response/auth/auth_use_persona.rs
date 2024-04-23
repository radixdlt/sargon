use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionAuthUsePersonaRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
}

impl HasSampleValues
    for WalletToDappInteractionAuthUsePersonaRequestResponseItem
{
    fn sample() -> Self {
        Self {
            persona: DappWalletInteractionPersona::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            persona: DappWalletInteractionPersona::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionAuthUsePersonaRequestResponseItem;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
