use crate::prelude::*;
use profile_logic::prelude::SavedGatewaysChangeCurrent as _;
use sargon::SavedGateways as InternalSavedGateways;

decl_vec_samples_for!(Gateways, Gateway);

/// The currently used Gateway and a collection of other by user added
/// or predefined Gateways the user can switch to.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct SavedGateways {
    /// The currently used Gateway, when a user query's asset balances of
    /// accounts or submits transactions, this Gateway will be used.
    pub current: Gateway,

    /// Other by user added or predefined Gateways the user can switch to.
    /// It might be Gateways with different URLs on the SAME network, or
    /// other networks, the identifier of a Gateway is the URL.
    pub other: Vec<Gateway>,
}

/// Constructs `Gateways` with `current` set as active Gateway.
#[uniffi::export]
pub fn new_saved_gateways(current: Gateway) -> SavedGateways {
    InternalSavedGateways::new(current.into()).into()
}

/// Constructs `Gateways` with default preset values.
#[uniffi::export]
pub fn new_saved_gateways_default() -> SavedGateways {
    InternalSavedGateways::default().into()
}

/// A sample value useful for tests and previews.
#[uniffi::export]
pub fn new_saved_gateways_sample() -> SavedGateways {
    InternalSavedGateways::sample().into()
}

/// A sample value useful for tests and previews.
#[uniffi::export]
pub fn new_saved_gateways_sample_other() -> SavedGateways {
    InternalSavedGateways::sample_other().into()
}

/// Returns the current and the other gateways of `gateways`.
#[uniffi::export]
pub fn saved_gateways_get_all_elements(
    gateways: &SavedGateways,
) -> Vec<Gateway> {
    gateways.into_internal().all().into_type()
}

#[uniffi::export]
pub fn new_saved_gateways_changing_current(
    to: Gateway,
    gateways: &SavedGateways,
) -> Result<SavedGateways> {
    let mut gateways = gateways.clone().into_internal();
    let _ = gateways.change_current(to.into());
    Ok(gateways.into())
}
