use crate::prelude::*;
use sargon::NewlyCreatedResource as InternalNewlyCreatedResource;

/// Metadata about a newly created Resource
#[derive(Clone, Debug, Default, PartialEq, Eq,  uniffi::Record)]
pub struct NewlyCreatedResource {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub tags: Vec<String>,
}

impl From<InternalNewlyCreatedResource> for NewlyCreatedResource {
    fn from(value: InternalNewlyCreatedResource) -> Self {
        Self {
            name: value.name,
            symbol: value.symbol,
            description: value.description,
            icon_url: value.icon_url,
            tags: value.tags,
        }
    }
}

impl Into<InternalNewlyCreatedResource> for NewlyCreatedResource {
    fn into(self) -> InternalNewlyCreatedResource {
        InternalNewlyCreatedResource {
            name: self.name,
            symbol: self.symbol,
            description: self.description,
            icon_url: self.icon_url,
            tags: self.tags,
        }
    }
}