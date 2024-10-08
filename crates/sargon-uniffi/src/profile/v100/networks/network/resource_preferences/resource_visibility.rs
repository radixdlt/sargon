use crate::prelude::*;
use sargon::ResourceVisibility as InternalResourceVisibility;

/// Indicates the visibility of a resource.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum ResourceVisibility {
    Hidden,
    Visible,
}

impl From<InternalResourceVisibility> for ResourceVisibility {
    fn from(value: InternalResourceVisibility) -> Self {
        match value {
            InternalResourceVisibility::Hidden => ResourceVisibility::Hidden,
            InternalResourceVisibility::Visible => ResourceVisibility::Visible,
        }
    }
}

impl Into<InternalResourceVisibility> for ResourceVisibility {
    fn into(self) -> InternalResourceVisibility {
        match self {
            ResourceVisibility::Hidden => InternalResourceVisibility::Hidden,
            ResourceVisibility::Visible => InternalResourceVisibility::Visible,
        }
    }
}