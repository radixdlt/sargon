use crate::prelude::*;
use sargon::DappToWalletInteractionUnvalidated as InternalDappToWalletInteractionUnvalidated;

json_data_convertible!(DappToWalletInteractionUnvalidated);

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionUnvalidated {
    pub interaction_id: WalletInteractionId,
    pub items: DappToWalletInteractionItems,
    pub metadata: DappToWalletInteractionMetadataUnvalidated,
}

#[uniffi::export]
pub fn new_dapp_to_wallet_interaction_unvalidated_from_json_string(
    json_str: String,
) -> Result<DappToWalletInteractionUnvalidated> {
    InternalDappToWalletInteractionUnvalidated::new_from_json_string(json_str)
        .into_result()
}

#[uniffi::export]
pub fn dapp_to_wallet_interaction_unvalidated_to_json_string(
    interaction_unvalidated: &DappToWalletInteractionUnvalidated,
) -> String {
    interaction_unvalidated
        .into_internal()
        .to_json_string(false)
}

#[uniffi::export]
pub(crate) fn new_dapp_to_wallet_interaction_unvalidated_sample(
) -> DappToWalletInteractionUnvalidated {
    InternalDappToWalletInteractionUnvalidated::sample().into()
}

#[uniffi::export]
pub(crate) fn new_dapp_to_wallet_interaction_unvalidated_sample_other(
) -> DappToWalletInteractionUnvalidated {
    InternalDappToWalletInteractionUnvalidated::sample_other().into()
}

#[cfg(test)]
mod test {

    #[test]
    fn test_dapp_to_wallet_interaction_unvalidated() {
        // let dapp_to_wallet_interaction_unvalidated =
        //     DappToWalletInteractionUnvalidated::sample();
        // let json_string = dapp_to_wallet_interaction_unvalidated.to_json_string();
        // let new_dapp_to_wallet_interaction_unvalidated =
        //     DappToWalletInteractionUnvalidated::from_json_string(&json_string).unwrap();
        // assert_eq!(
        //     dapp_to_wallet_interaction_unvalidated,
        //     new_dapp_to_wallet_interaction_unvalidated
        // );
    }
}
