use crate::prelude::*;
use sargon::ChildIntents as InternalChildIntents;

/// Represents a collection of child subintents.
///
/// This struct is used to manage a list of `ChildSubintent` instances, providing
/// methods for creation, conversion, and sample values for testing purposes.
///
/// # Fields
/// - `children`: A vector of `ChildSubintent` instances.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct ChildIntents {
    pub children: Vec<ChildSubintent>,
}
