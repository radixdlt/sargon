use crate::prelude::*;
use sargon::DappToWalletInteraction as InternalDappToWalletInteraction;

#[derive(Debug, Clone, PartialEq,  uniffi::Record)]
pub struct DappToWalletInteraction {
    pub interaction_id: WalletInteractionId,
    pub items: DappToWalletInteractionItems,
    pub metadata: DappToWalletInteractionMetadata,
}

impl From<InternalDappToWalletInteraction> for DappToWalletInteraction {
    fn from(value: InternalDappToWalletInteraction) -> Self {
        Self {
            interaction_id: value.interaction_id.into(),
            items: value.items.into(),
            metadata: value.metadata.into(),
        }
    }
}

impl Into<InternalDappToWalletInteraction> for DappToWalletInteraction {
    fn into(self) -> InternalDappToWalletInteraction {
        InternalDappToWalletInteraction {
            interaction_id: self.interaction_id.into(),
            items: self.items.into(),
            metadata: self.metadata.into(),
        }
    }
}

#[uniffi::export]
pub fn new_dapp_to_wallet_interaction_unvalidated_from_json_string(
    json_str: String,
) -> Result<DappToWalletInteractionUnvalidated> {
    InternalDappToWalletInteraction::new_from_json_string(json_str).map_result()
}

#[uniffi::export]
pub fn dapp_to_wallet_interaction_unvalidated_to_json_string(
    interaction_unvalidated: &DappToWalletInteractionUnvalidated,
    pretty_printed: bool,
) -> String {
    interaction_unvalidated.into_internal().to_json_string(pretty_printed)
}

#[uniffi::export]
pub(crate) fn new_dapp_to_wallet_interaction_unvalidated_sample(
) -> DappToWalletInteractionUnvalidated {
    InternalDappToWalletInteraction::sample().into()
}

#[uniffi::export]
pub(crate) fn new_dapp_to_wallet_interaction_unvalidated_sample_other(
) -> DappToWalletInteractionUnvalidated {
    InternalDappToWalletInteraction::sample_other().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionUnvalidated;

    #[test]
    fn inequality_of_samples() {
        assert_ne!(
            new_dapp_to_wallet_interaction_unvalidated_sample(),
            new_dapp_to_wallet_interaction_unvalidated_sample_other()
        );
    }

    #[test]
    fn json_string_roundtrip() {
        let sut = SUT::sample();
        let pretty_string =
            dapp_to_wallet_interaction_unvalidated_to_json_string(&sut, true);
        let from_str =
            new_dapp_to_wallet_interaction_unvalidated_from_json_string(
                pretty_string.clone(),
            )
            .unwrap();
        assert_eq!(from_str, sut);
        let ugly_string =
            dapp_to_wallet_interaction_unvalidated_to_json_string(&sut, false);
        let from_str =
            new_dapp_to_wallet_interaction_unvalidated_from_json_string(
                ugly_string.clone(),
            )
            .unwrap();
        assert_eq!(from_str, sut);
        assert_ne!(pretty_string, ugly_string);
    }

    #[test]
    fn from_invalid_json_string_throws() {
        assert_eq!(
            new_dapp_to_wallet_interaction_unvalidated_from_json_string(
                "".to_owned()
            ),
            Err(CommonError::FailedToDeserializeJSONToValue {
                json_byte_count: 0,
                type_name: "DappToWalletInteractionUnvalidated".to_owned(),
                serde_message: "EOF while parsing a value at line 1 column 0"
                    .to_string()
            })
        )
    }
}