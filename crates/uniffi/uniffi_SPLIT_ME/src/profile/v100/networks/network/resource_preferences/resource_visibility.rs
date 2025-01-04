use crate::prelude::*;
use sargon::ResourceVisibility as InternalResourceVisibility;

decl_vec_samples_for!(HiddenResources, ResourceIdentifier);

/// Indicates the visibility of a resource.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum ResourceVisibility {
    Hidden,
    Visible,
}
