use crate::prelude::*;

json_data_convertible!(DappToWalletInteractionUnvalidated);
json_data_convertible!(WalletToDappInteractionResponse);

#[uniffi::export]
pub fn new_dapp_to_wallet_interaction_unvalidated_from_json_string(
    json_str: String,
) -> Result<DappToWalletInteractionUnvalidated> {
    DappToWalletInteractionUnvalidated::new_from_json_string(json_str)
}

impl DappToWalletInteractionUnvalidated {
    pub fn new_from_json_string(
        json_str: impl AsRef<str>,
    ) -> Result<DappToWalletInteractionUnvalidated> {
        let json_str = json_str.as_ref();
        let json_byte_count = json_str.len() as u64;
        serde_json::from_str(json_str).map_err(|_| {
            CommonError::FailedToDeserializeJSONToValue {
                json_byte_count,
                type_name: type_name::<DappToWalletInteractionUnvalidated>(),
            }
        })
    }
}

#[uniffi::export]
pub fn dapp_to_wallet_interaction_unvalidated_to_json_string(
    interaction_unvalidated: &DappToWalletInteractionUnvalidated,
    pretty_printed: bool,
) -> String {
    interaction_unvalidated.to_json_string(pretty_printed)
}

impl DappToWalletInteractionUnvalidated {
    pub fn to_json_string(&self, pretty_printed: bool) -> String {
        if pretty_printed {
            serde_json::to_string_pretty(self)
        } else {
            serde_json::to_string(self)
        }
        .expect("Should always be able to JSON encode DappToWalletInteractionUnvalidated.")
    }
}

#[uniffi::export]
pub(crate) fn new_dapp_to_wallet_interaction_unvalidated_sample(
) -> DappToWalletInteractionUnvalidated {
    DappToWalletInteractionUnvalidated::sample()
}

#[uniffi::export]
pub(crate) fn new_dapp_to_wallet_interaction_unvalidated_sample_other(
) -> DappToWalletInteractionUnvalidated {
    DappToWalletInteractionUnvalidated::sample_other()
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
                type_name: "DappToWalletInteractionUnvalidated".to_owned()
            })
        )
    }
}
