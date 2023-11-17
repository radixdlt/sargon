use std::cell::Cell;

use serde::{Deserialize, Serialize};

/// A hint describing the contents of a Profile, acting as a
/// summary of a Profile used by a ProfileSnapshot Header.
///
/// Important to know that this is just a **hint**, the values
/// SHOULD be kept up to date, might might not be, since they
/// are stored values which must be kept in sync.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
    pub number_of_accounts_on_all_networks_in_total: Cell<u32>,

    /// The total number of personas on all networks.
    ///
    /// Important to remember that this is a counter inside a
    /// content **hint**. This counter SHOULD be update when
    /// new accounts are created, but failing to do is of no
    /// real consequence.
    ///
    /// This counter includes any by user hidden personas.
    #[serde(rename = "numberOfPersonasOnAllNetworksInTotal")]
    pub number_of_personas_on_all_networks_in_total: Cell<u32>,

    /// The total number of networks that the user has used, i.e.
    /// on which she has any accounts or personas.
    #[serde(rename = "numberOfNetworks")]
    pub number_of_networks: Cell<u32>,
}

impl ContentHint {
    /// Instantiates a new `ContentHint` with the specified counter values.
    pub fn with_counters(accounts: u32, personas: u32, networks: u32) -> Self {
        Self {
            number_of_accounts_on_all_networks_in_total: Cell::new(accounts),
            number_of_personas_on_all_networks_in_total: Cell::new(personas),
            number_of_networks: Cell::new(networks),
        }
    }

    /// Instantiates a new empty ContentHint with all counters equal `0`.
    pub fn new() -> Self {
        Self::with_counters(0, 0, 0)
    }
}

#[cfg(test)]
mod tests {

    use super::ContentHint;

    #[test]
    fn new_counters_all_start_at_zero() {
        let sut = ContentHint::new();
        assert_eq!(sut.number_of_accounts_on_all_networks_in_total.get(), 0);
        assert_eq!(sut.number_of_personas_on_all_networks_in_total.get(), 0);
        assert_eq!(sut.number_of_networks.get(), 0);
    }

    #[test]
    fn with_counters_x_y_z() {
        (0..100).into_iter().for_each(|i| {
            let x = i + 1;
            let y = i + 2;
            let z = i + 3;
            let sut = ContentHint::with_counters(x, y, z);
            assert_eq!(sut.number_of_accounts_on_all_networks_in_total.get(), x);
            assert_eq!(sut.number_of_personas_on_all_networks_in_total.get(), y);
            assert_eq!(sut.number_of_networks.get(), z)
        });
    }

    // #[test_each(0..9)]
    // fn with_counters_x_y_z_macro(i: u32) {
    //     let x = i + 1;
    //     let y = i + 2;
    //     let z = i + 3;
    //     let sut = ContentHint::with_counters(x, y, z);
    //     assert_eq!(sut.number_of_accounts_on_all_networks_in_total.get(), x);
    //     assert_eq!(sut.number_of_personas_on_all_networks_in_total.get(), y);
    //     assert_eq!(sut.number_of_networks.get(), z)
    // }
}
