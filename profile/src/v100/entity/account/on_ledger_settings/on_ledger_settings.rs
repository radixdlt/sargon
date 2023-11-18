use std::cell::{Cell, RefCell};

use serde::{Deserialize, Serialize};

use super::third_party_deposits::third_party_deposits::ThirdPartyDeposits;

/// Account settings that user has set on the account component
/// On-Ledger, that is set via a transaction mutating the state
/// on the network.
///
/// This settings include third-party deposits, controlling who
/// can send which assets to this account.
///
/// These settings SHOULD be kept in sync between local state
/// (in Profile) and On-Ledger.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct OnLedgerSettings {
    /// Controls the ability of third-parties to deposit into this account
    third_party_deposits: RefCell<ThirdPartyDeposits>,
}
