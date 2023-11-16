use serde::{Deserialize, Serialize};

/// A hint describing the contents of a Profile, acting as a summary of a Profile used
/// by a ProfileSnapshot Header.
///
/// Important to know that this is just a **hint**, the values
/// SHOULD be kept up to date, might might not be, since they
/// are stored values which must be kept in sync.
#[derive(Serialize, Deserialize)]
pub struct ContentHint {
    /// The total number of accounts on all networks.
    ///
    /// Important to remember that this is a counter inside a
    /// content **hint**. This counter SHOULD be update when
    /// new accounts are created, but failing to do is of no
    /// real consequence.
    ///
    /// This counter includes any by user hidden accounts.
    #[serde(rename = "numberOfAccountsOnAllNetworksInTotal")]
    pub number_of_accounts_on_all_networks_in_total: u32,

    /// The total number of personas on all networks.
    ///
    /// Important to remember that this is a counter inside a
    /// content **hint**. This counter SHOULD be update when
    /// new accounts are created, but failing to do is of no
    /// real consequence.
    ///
    /// This counter includes any by user hidden personas.
    #[serde(rename = "numberOfPersonasOnAllNetworksInTotal")]
    pub number_of_personas_on_all_networks_in_total: u32,

    /// The total number of networks that the user has used, i.e.
    /// on which she has any accounts or personas.
    #[serde(rename = "numberOfNetworks")]
    pub number_of_networks: u32,
}
