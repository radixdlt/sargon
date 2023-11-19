use serde::{Deserialize, Serialize};

use crate::v100::address::resource_address::ResourceAddress;

use super::deposit_address_exception_rule::DepositAddressExceptionRule;

/// The specific Asset exception rule
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct AssetException {
    pub address: ResourceAddress,
    pub exception_rule: DepositAddressExceptionRule,
}

impl AssetException {
    pub fn new(address: ResourceAddress, exception_rule: DepositAddressExceptionRule) -> Self {
        Self {
            address,
            exception_rule,
        }
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use crate::v100::entity::account::on_ledger_settings::third_party_deposits::deposit_address_exception_rule::DepositAddressExceptionRule;

    use super::AssetException;

    #[test]
    fn json_decode_deny_all_with_exceptions() {
        let model = AssetException::new(
            "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"
                .try_into()
                .unwrap(),
            DepositAddressExceptionRule::Allow,
        );
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "address" : "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq",
                "exceptionRule" : "allow"
            }
            "#,
        )
    }
}
