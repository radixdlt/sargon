use crate::prelude::*;

/// A preference the user has configured off-ledger for a given resource.
/// Allows users, for example, to hide a given resource on their accounts.
///
/// Named like this to differ from RET's `ResourcePreference`.
#[derive(
    Clone, PartialEq, Eq, Debug, Hash, uniffi::Record,
)]
pub struct ResourceAppPreference {
    /// The resource for which the preference is set up.
    pub resource: ResourceIdentifier,

    /// The visibility of the resource (hidden or visible).
    pub visibility: ResourceVisibility,
}

impl Identifiable for ResourceAppPreference {
    type ID = ResourceIdentifier;
    fn id(&self) -> Self::ID {
        self.resource.clone()
    }
}