use crate::prelude::*;
use sargon::ResourceVisibility as InternalResourceVisibility;

/// Indicates the visibility of a resource.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum ResourceVisibility {
    Hidden,
    Visible,
}
