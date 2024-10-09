use crate::prelude::*;
use sargon::ContentHint as InternalContentHint;

/// A hint describing the contents of a Profile, acting as a
/// summary of a Profile used by a ProfileSnapshot Header.
///
/// Important to know that this is just a **hint**, the values
/// SHOULD be kept up to date, might might not be, since they
/// are stored values which must be kept in sync.
#[derive(
    Clone,
    
    Debug,
    PartialEq,
    Eq,
    Hash,
     uniffi::Record,
)]
pub struct ContentHint {
    /// The total number of accounts on all networks.
    ///
    /// Important to remember that this is a counter inside a
    /// content **hint**. This counter SHOULD be update when
    /// new accounts are created, but failing to do is of no
    /// real consequence.
    ///
    /// This counter includes any by user hidden accounts.
    pub number_of_accounts_on_all_networks_in_total: u16,

    /// The total number of personas on all networks.
    ///
    /// Important to remember that this is a counter inside a
    /// content **hint**. This counter SHOULD be update when
    /// new accounts are created, but failing to do is of no
    /// real consequence.
    ///
    /// This counter includes any by user hidden personas.
    pub number_of_personas_on_all_networks_in_total: u16,

    /// The total number of networks that the user has used, i.e.
    /// on which she has any accounts or personas.
    pub number_of_networks: u16,
}

impl From<InternalContentHint> for ContentHint {
    fn from(value: InternalContentHint) -> Self {
        Self {
            number_of_accounts_on_all_networks_in_total: value.number_of_accounts_on_all_networks_in_total,
            number_of_personas_on_all_networks_in_total: value.number_of_personas_on_all_networks_in_total,
            number_of_networks: value.number_of_networks,
        }
    }
}

impl Into<InternalContentHint> for ContentHint {
    fn into(self) -> InternalContentHint {
        InternalContentHint {
            number_of_accounts_on_all_networks_in_total: self.number_of_accounts_on_all_networks_in_total,
            number_of_personas_on_all_networks_in_total: self.number_of_personas_on_all_networks_in_total,
            number_of_networks: self.number_of_networks,
        }
    }
}