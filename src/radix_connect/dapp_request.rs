use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Record)]
pub struct DappRequest {
    pub interaction_id: WalletInteractionId,
    pub session_id: SessionID,
}

impl DappRequest {
    pub fn new(
        interaction_id: WalletInteractionId,
        session_id: SessionID,
    ) -> Self {
        Self {
            interaction_id,
            session_id,
        }
    }

    pub(crate) fn try_with_interaction_id_and_session_id(
        interaction_id: impl AsRef<str>,
        session_id: impl AsRef<str>,
    ) -> Result<Self> {
        let interaction_id = WalletInteractionId::from_str(
            interaction_id.as_ref(),
        )
        .map_err(|_| CommonError::RadixMobileInvalidInteractionID {
            bad_value: interaction_id.as_ref().to_owned(),
        })?;
        let session_id =
            SessionID::from_str(session_id.as_ref()).map_err(|_| {
                CommonError::RadixConnectMobileInvalidSessionID {
                    bad_value: session_id.as_ref().to_owned(),
                }
            })?;
        Ok(DappRequest::new(interaction_id, session_id))
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
        let session_id = Uuid::new_v4().to_string();
        let interaction_id = Uuid::new_v4().to_string();
        let sut = SUT::try_with_interaction_id_and_session_id(
            interaction_id.clone(),
            session_id.clone(),
        )
        .unwrap();
        assert_eq!(sut.interaction_id.0.to_string(), interaction_id);
        assert_eq!(sut.session_id.0.to_string(), session_id);
    }

    #[test]
    fn try_with_invalid_interaction_id() {
        let session_id = Uuid::new_v4().to_string();
        let interaction_id = "bad";
        assert_eq!(
            SUT::try_with_interaction_id_and_session_id(
                interaction_id,
                session_id.clone()
            ),
            Err(CommonError::RadixMobileInvalidInteractionID {
                bad_value: interaction_id.to_owned()
            })
        );
    }

    #[test]
    fn try_with_invalid_session_id() {
        let session_id = "bad";
        let interaction_id = Uuid::new_v4().to_string();
        assert_eq!(
            SUT::try_with_interaction_id_and_session_id(
                interaction_id.clone(),
                session_id
            ),
            Err(CommonError::RadixConnectMobileInvalidSessionID {
                bad_value: session_id.to_owned()
            })
        );
    }

    #[test]
    fn test_new() {
        let session_id = SessionID::sample();
        let interaction_id = WalletInteractionId::sample();
        let sut = SUT::new(interaction_id.clone(), session_id.clone());
        assert_eq!(sut.interaction_id, interaction_id);
        assert_eq!(sut.session_id, session_id);
    }
}
