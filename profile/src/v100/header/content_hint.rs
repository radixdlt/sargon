use std::{cell::Cell, fmt::Display};

use serde::{Deserialize, Serialize};

/// A hint describing the contents of a Profile, acting as a
/// summary of a Profile used by a ProfileSnapshot Header.
///
/// Important to know that this is just a **hint**, the values
/// SHOULD be kept up to date, might might not be, since they
/// are stored values which must be kept in sync.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContentHint {
    /// The total number of accounts on all networks.
    ///
    /// Important to remember that this is a counter inside a
    /// content **hint**. This counter SHOULD be update when
    /// new accounts are created, but failing to do is of no
    /// real consequence.
    ///
    /// This counter includes any by user hidden accounts.
    number_of_accounts_on_all_networks_in_total: Cell<u32>,

    /// The total number of personas on all networks.
    ///
    /// Important to remember that this is a counter inside a
    /// content **hint**. This counter SHOULD be update when
    /// new accounts are created, but failing to do is of no
    /// real consequence.
    ///
    /// This counter includes any by user hidden personas.
    number_of_personas_on_all_networks_in_total: Cell<u32>,

    /// The total number of networks that the user has used, i.e.
    /// on which she has any accounts or personas.
    number_of_networks: Cell<u32>,
}

// Constructors
impl ContentHint {
    /// Instantiates a new `ContentHint` with the specified counter values.
    pub fn with_counters(accounts: u32, personas: u32, networks: u32) -> Self {
        Self {
            number_of_accounts_on_all_networks_in_total: Cell::new(accounts),
            number_of_personas_on_all_networks_in_total: Cell::new(personas),
            number_of_networks: Cell::new(networks),
        }
    }

    /// Instantiates a new `ContentHint` with all counters equal `count`.
    pub(crate) fn all(count: u32) -> Self {
        Self::with_counters(count, count, count)
    }

    /// Instantiates a new empty ContentHint with all counters equal `0`.
    pub fn new() -> Self {
        Self::all(0)
    }
}

impl Default for ContentHint {
    /// Instantiates a new empty ContentHint with all counters equal `0`.
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ContentHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#networks: {}, #accounts: {}, #personas: {}",
            self.get_number_of_networks(),
            self.get_number_of_accounts_on_all_networks_in_total(),
            self.get_number_of_personas_on_all_networks_in_total()
        )
    }
}

// Getters
impl ContentHint {
    /// Gets the number of accounts on all networks in total.
    pub fn get_number_of_accounts_on_all_networks_in_total(&self) -> u32 {
        self.number_of_accounts_on_all_networks_in_total.get()
    }

    /// Gets the number of personas on all networks in total.
    pub fn get_number_of_personas_on_all_networks_in_total(&self) -> u32 {
        self.number_of_personas_on_all_networks_in_total.get()
    }

    /// Gets the number of networks.
    pub fn get_number_of_networks(&self) -> u32 {
        self.number_of_networks.get()
    }
}

// Setters
impl ContentHint {
    /// Sets the `number_of_accounts_on_all_networks_in_total` to `new`.
    pub fn set_number_of_accounts_on_all_networks_in_total(&self, new: u32) {
        self.number_of_accounts_on_all_networks_in_total.set(new)
    }

    /// Sets the `number_of_personas_on_all_networks_in_total` to `new`.
    pub fn set_number_of_personas_on_all_networks_in_total(&self, new: u32) {
        self.number_of_personas_on_all_networks_in_total.set(new)
    }

    /// Sets the `number_of_networks` to `new`.
    pub fn set_number_of_networks(&self, new: u32) {
        self.number_of_networks.set(new)
    }
}

#[cfg(test)]
mod tests {

    use wallet_kit_test_utils::json::{
        assert_eq_after_json_roundtrip, assert_json_fails, assert_json_roundtrip,
        assert_ne_after_json_roundtrip,
    };

    use super::ContentHint;

    #[test]
    fn new_counters_all_start_at_zero() {
        let sut = ContentHint::new();
        assert_eq!(sut.number_of_accounts_on_all_networks_in_total.get(), 0);
        assert_eq!(sut.number_of_personas_on_all_networks_in_total.get(), 0);
        assert_eq!(sut.number_of_networks.get(), 0);
    }

    #[test]
    fn eq() {
        assert_eq!(ContentHint::new(), ContentHint::new());
    }

    #[test]
    fn display() {
        let sut = ContentHint::default();
        assert_eq!(
            format!("{}", sut),
            "#networks: 0, #accounts: 0, #personas: 0"
        )
    }

    #[test]
    fn getters() {
        (0..100).into_iter().for_each(|i| {
            let x = i + 1;
            let y = i + 2;
            let z = i + 3;
            let sut = ContentHint::with_counters(x, y, z);
            assert_eq!(sut.get_number_of_accounts_on_all_networks_in_total(), x);
            assert_eq!(sut.get_number_of_personas_on_all_networks_in_total(), y);
            assert_eq!(sut.get_number_of_networks(), z)
        });
    }

    #[test]
    fn set_number_of_accounts_on_all_networks_in_total() {
        let sut = ContentHint::new();
        (1..100).into_iter().for_each(|i| {
            assert_eq!(sut.get_number_of_accounts_on_all_networks_in_total(), i - 1);
            sut.set_number_of_accounts_on_all_networks_in_total(i);

            // assert rest unchanged
            assert_eq!(sut.get_number_of_networks(), 0);
            assert_eq!(sut.get_number_of_personas_on_all_networks_in_total(), 0);
        });
    }

    #[test]
    fn set_number_of_personas_on_all_networks_in_total() {
        let sut = ContentHint::new();
        (1..100).into_iter().for_each(|i| {
            assert_eq!(sut.get_number_of_personas_on_all_networks_in_total(), i - 1);
            sut.set_number_of_personas_on_all_networks_in_total(i);

            // assert rest unchanged
            assert_eq!(sut.get_number_of_networks(), 0);
            assert_eq!(sut.get_number_of_accounts_on_all_networks_in_total(), 0);
        });
    }

    #[test]
    fn set_number_of_networks() {
        let sut = ContentHint::new();
        (1..100).into_iter().for_each(|i| {
            assert_eq!(sut.get_number_of_networks(), i - 1);
            sut.set_number_of_networks(i);

            // assert rest unchanged
            assert_eq!(sut.get_number_of_accounts_on_all_networks_in_total(), 0);
            assert_eq!(sut.get_number_of_personas_on_all_networks_in_total(), 0);
        });
    }

    #[test]
    fn json_roundtrip() {
        let model = ContentHint::with_counters(3, 2, 1);
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "numberOfAccountsOnAllNetworksInTotal": 3,
                "numberOfPersonasOnAllNetworksInTotal": 2,
                "numberOfNetworks": 1
            }
            "#,
        );
        assert_json_roundtrip(&model);
        assert_ne_after_json_roundtrip(
            &model,
            r#"
            {
                "numberOfAccountsOnAllNetworksInTotal": 1337,
                "numberOfPersonasOnAllNetworksInTotal": 237,
                "numberOfNetworks": 42
            }
            "#,
        );
    }

    #[test]
    fn invalid_json() {
        assert_json_fails::<ContentHint>(
            r#"
            {
                "numberOfAccountsOnAllNetworksInTotal": "oh a string",
                "numberOfPersonasOnAllNetworksInTotal": 237,
                "numberOfNetworks": 42
            }
            "#,
        );

        assert_json_fails::<ContentHint>(
            r#"
            {
                "missing_key": "numberOfAccountsOnAllNetworksInTotal",
                "numberOfPersonasOnAllNetworksInTotal": 237,
                "numberOfNetworks": 42
            }
            "#,
        );

        assert_json_fails::<ContentHint>(
            r#"
            {
                "numberOfAccountsOnAllNetworksInTotal": 1337,
                "missing_key": "numberOfPersonasOnAllNetworksInTotal",
                "numberOfNetworks": 42
            }
            "#,
        );

        assert_json_fails::<ContentHint>(
            r#"
            {
                "numberOfAccountsOnAllNetworksInTotal": 1337,
                "numberOfPersonasOnAllNetworksInTotal": 237,
                "missing_key": "numberOfNetworks"
            }
            "#,
        );

        // We are NOT doing snake case
        assert_json_fails::<ContentHint>(
            r#"
            {
                "number_of_accounts_on_all_networks_in_total": 1337,
                "number_of_personas_on_all_networks_in_total": 237,
                "number_of_networks": 42
            }
            "#,
        );
    }
}
