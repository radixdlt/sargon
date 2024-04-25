use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Record)]
pub struct DappRequest {
    pub interaction_id: WalletInteractionId,
    pub session_id: SessionID,
}

impl DappRequest {
    pub(crate) fn try_with_interaction_id_and_session_id(
        interaction_id: impl AsRef<str>,
        session_id: SessionID,
    ) -> Result<Self> {
        let interaction_id = WalletInteractionId::new(interaction_id);
        Ok(Self {
            interaction_id,
            session_id,
        })
    }
}

impl HasSampleValues for DappRequest {
    fn sample() -> Self {
        Self {
            interaction_id: WalletInteractionId::sample(),
            session_id: SessionID::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            interaction_id: WalletInteractionId::sample_other(),
            session_id: SessionID::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappRequest;

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
    fn try_with_interaction_id_and_session_id() {
        let session_id = SessionID::sample();
        let interaction_id = "interaction_id";
        let sut = SUT::try_with_interaction_id_and_session_id(
            interaction_id,
            session_id.clone(),
        )
        .unwrap();
        assert_eq!(
            sut.interaction_id,
            WalletInteractionId::new(interaction_id)
        );
        assert_eq!(sut.session_id, session_id);
    }
}
