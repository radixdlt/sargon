use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Record,
)]
pub struct AccountDepositSettingsUpdateAnalyzedManifest {
    pub resource_preferences_updates: HashMap<
        AccountAddress,
        HashMap<ResourceAddress, ResourcePreferenceUpdate>,
    >,
    pub deposit_mode_updates:
        HashMap<AccountAddress, AccountDefaultDepositRule>,
    pub authorized_depositors_added:
        HashMap<AccountAddress, Vec<ResourceOrNonFungible>>,
    pub authorized_depositors_removed:
        HashMap<AccountAddress, Vec<ResourceOrNonFungible>>,
}