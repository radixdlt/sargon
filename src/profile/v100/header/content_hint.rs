use crate::prelude::*;
/// A hint describing the contents of a Profile, acting as a
/// summary of a Profile used by a ProfileSnapshot Header.
///
/// Important to know that this is just a **hint**, the values
/// SHOULD be kept up to date, might might not be, since they
/// are stored values which must be kept in sync.
#[derive(
    Serialize,
    Deserialize,
    Default,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[display("#networks: {number_of_networks}, #accounts: {number_of_accounts_on_all_networks_in_total}, #personas: {number_of_personas_on_all_networks_in_total}")]
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

// Constructors
impl ContentHint {
    /// Instantiates a new `ContentHint` with the specified counter values.
    pub fn with_counters(
        accounts: usize,
        personas: usize,
        networks: usize,
    ) -> Self {
        Self {
            number_of_accounts_on_all_networks_in_total: accounts as u16,
            number_of_personas_on_all_networks_in_total: personas as u16,
            number_of_networks: networks as u16,
        }
    }

    /// Instantiates a new `ContentHint` with all counters equal `count`.
    pub(crate) fn all(count: usize) -> Self {
        Self::with_counters(count, count, count)
    }

    /// Instantiates a new empty ContentHint with all counters equal `0`.
    pub fn new() -> Self {
        Self::all(0)
    }
}

impl HasSampleValues for ContentHint {
    fn sample() -> Self {
        Self::with_counters(3, 1, 1)
    }
    fn sample_other() -> Self {
        Self::with_counters(1, 0, 0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ContentHint;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn new_counters_all_start_at_zero() {
        let sut = SUT::new();
        assert_eq!(sut.number_of_accounts_on_all_networks_in_total, 0);
        assert_eq!(sut.number_of_personas_on_all_networks_in_total, 0);
        assert_eq!(sut.number_of_networks, 0);
    }

    #[test]
    fn display() {
        let sut = SUT::default();
        assert_eq!(
            format!("{}", sut),
            "#networks: 0, #accounts: 0, #personas: 0"
        )
    }

    #[test]
    fn with_counters_constructor() {
        (0..100).for_each(|i| {
            let x = i + 1;
            let y = i + 2;
            let z = i + 3;
            let sut = SUT::with_counters(x, y, z);
            assert_eq!(
                sut.number_of_accounts_on_all_networks_in_total as usize,
                x
            );
            assert_eq!(
                sut.number_of_personas_on_all_networks_in_total as usize,
                y
            );
            assert_eq!(sut.number_of_networks as usize, z)
        });
    }

    #[test]
    fn json_roundtrip() {
        let model = SUT::with_counters(3, 2, 1);
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
        assert_json_fails::<SUT>(
            r#"
            {
                "numberOfAccountsOnAllNetworksInTotal": "oh a string",
                "numberOfPersonasOnAllNetworksInTotal": 237,
                "numberOfNetworks": 42
            }
            "#,
        );

        assert_json_fails::<SUT>(
            r#"
            {
                "missing_key": "numberOfAccountsOnAllNetworksInTotal",
                "numberOfPersonasOnAllNetworksInTotal": 237,
                "numberOfNetworks": 42
            }
            "#,
        );

        assert_json_fails::<SUT>(
            r#"
            {
                "numberOfAccountsOnAllNetworksInTotal": 1337,
                "missing_key": "numberOfPersonasOnAllNetworksInTotal",
                "numberOfNetworks": 42
            }
            "#,
        );

        assert_json_fails::<SUT>(
            r#"
            {
                "numberOfAccountsOnAllNetworksInTotal": 1337,
                "numberOfPersonasOnAllNetworksInTotal": 237,
                "missing_key": "numberOfNetworks"
            }
            "#,
        );

        // We are NOT doing snake case
        assert_json_fails::<SUT>(
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
