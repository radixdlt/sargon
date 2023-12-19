use std::cell::RefCell;

use serde::{Deserialize, Serialize};

use super::{
    app_preferences::app_preferences::AppPreferences,
    factors::factor_sources::factor_sources::FactorSources, header::header::Header,
    networks::networks::Networks,
};

/// Representation of the Radix Wallet, contains a list of
/// users Accounts, Personas, Authorized Dapps per network
/// the user has used. It also contains all FactorSources,
/// FactorInstances and wallet App preferences.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// The header of a Profile(Snapshot) contains crucial metadata
    /// about this Profile, such as which JSON data format it is
    /// compatible with and which device was used to create it and
    /// a hint about its contents.
    header: RefCell<Header>,

    /// All sources of factors, used for authorization such as spending funds, contains no
    /// secrets.
    factor_sources: RefCell<FactorSources>,

    /// Settings for this profile in the app, contains default security configs
    /// as well as display settings.
    app_preferences: RefCell<AppPreferences>,

    /// An ordered mapping of NetworkID -> `Profile.Network`, containing
    /// all the users Accounts, Personas and AuthorizedDapps the user
    /// has created and interacted with on this network.
    networks: RefCell<Networks>,
}

impl Profile {
    /// Panics if `factor_sources` is empty, since FactorSources MUST not be empty.
    pub fn with(
        header: Header,
        factor_sources: FactorSources,
        app_preferences: AppPreferences,
        networks: Networks,
    ) -> Self {
        factor_sources.assert_not_empty();
        Self {
            header: RefCell::new(header),
            factor_sources: RefCell::new(factor_sources),
            app_preferences: RefCell::new(app_preferences),
            networks: RefCell::new(networks),
        }
    }
}

impl Profile {
    pub fn set_header(&self, new: Header) {
        *self.header.borrow_mut() = new
    }

    /// Panics if `new` is empty, since FactorSources MUST not be empty.
    pub fn set_factor_sources(&self, new: FactorSources) {
        new.assert_not_empty();
        *self.factor_sources.borrow_mut() = new
    }

    pub fn set_app_preferences(&self, new: AppPreferences) {
        *self.app_preferences.borrow_mut() = new
    }

    pub fn set_networks(&self, new: Networks) {
        *self.networks.borrow_mut() = new
    }
}
