use crate::prelude::*;
use radix_engine_interface::blueprints::account::{
    AccountRemoveAuthorizedDepositorInput as ScryptoAccountRemoveAuthorizedDepositorInput,
    AccountRemoveResourcePreferenceInput as ScryptoAccountRemoveResourcePreferenceInput,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ManifestRequest {
    DeleteAccount(ManifestRequestDeleteAccount),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestRequestDeleteAccount {
    pub account_address: AccountAddress,
    pub resource_preferences_to_be_removed:
        Vec<ScryptoAccountRemoveResourcePreferenceInput>,
    pub authorized_depositors_to_be_removed:
        Vec<ScryptoAccountRemoveAuthorizedDepositorInput>,
}
