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
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OnLedgerSettings {
    /// Controls the ability of third-parties to deposit into this account
    third_party_deposits: RefCell<ThirdPartyDeposits>,
}

#[cfg(test)]
mod tests {
    use wallet_kit_test_utils::json::{
        assert_eq_after_json_roundtrip, assert_json_roundtrip, assert_ne_after_json_roundtrip,
    };

    use super::OnLedgerSettings;

    #[test]
    fn json_roundtrip() {
        let model = OnLedgerSettings::default();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
            	"thirdPartyDeposits" : {
            		"assetsExceptionList" : [],
            		"depositorsAllowList" : [],
            		"depositRule" : "acceptAll"
            	}
            }
            "#,
        );
        assert_json_roundtrip(&model);
        assert_ne_after_json_roundtrip(
            &model,
            r#"
            {
            	"thirdPartyDeposits" : {
            		"assetsExceptionList" : [],
            		"depositorsAllowList" : [],
            		"depositRule" : "acceptKnown"
            	}
            }
            "#,
        );
    }
}
