use crate::prelude::*;
use sargon::ResourceAppPreference as InternalResourceAppPreference;

/// A preference the user has configured off-ledger for a given resource.
/// Allows users, for example, to hide a given resource on their accounts.
///
/// Named like this to differ from RET's `ResourcePreference`.
#[derive(
    Clone, PartialEq, Eq, Debug, Hash,  uniffi::Record,
)]
pub struct ResourceAppPreference {
    /// The resource for which the preference is set up.
    pub resource: ResourceIdentifier,

    /// The visibility of the resource (hidden or visible).
    pub visibility: ResourceVisibility,
}

impl From<InternalResourceAppPreference> for ResourceAppPreference {
    fn from(value: InternalResourceAppPreference) -> Self {
        Self {
            resource: value.resource.into(),
            visibility: value.visibility.into(),
        }
    }
}

impl Into<InternalResourceAppPreference> for ResourceAppPreference {
    fn into(self) -> InternalResourceAppPreference {
        InternalResourceAppPreference {
            resource: self.resource.into(),
            visibility: self.visibility.into(),
        }
    }
}