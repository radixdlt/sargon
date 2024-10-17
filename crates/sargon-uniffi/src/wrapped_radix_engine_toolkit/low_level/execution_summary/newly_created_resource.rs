use crate::prelude::*;
use sargon::NewlyCreatedResource as InternalNewlyCreatedResource;

/// Metadata about a newly created Resource
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct NewlyCreatedResource {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub tags: Vec<String>,
}
