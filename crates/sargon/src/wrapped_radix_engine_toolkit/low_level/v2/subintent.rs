use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, derive_more::Debug, uniffi::Record)]
pub struct Subintent {
    pub intent_core: IntentCoreV2,
}

impl From<Subintent> for ScryptoSubintent {
    fn from(value: Subintent) -> Self {
        Self {
            intent_core: value.intent_core.into(),
        }
    }
}

impl TryFrom<ScryptoSubintent> for Subintent {
    type Error = crate::CommonError;

    fn try_from(value: ScryptoSubintent) -> Result<Self> {
        Ok(Self {
            intent_core: value.intent_core.try_into()?,
        })
    }
}

impl HasSampleValues for Subintent {
    fn sample() -> Self {
        Self {
            intent_core: IntentCoreV2::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            intent_core: IntentCoreV2::sample_other(),
        }
    }
}
