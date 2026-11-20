use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FungibleResourcesCollectionItemVaultAggregatedVaultItem {
    /// Bech32m-encoded human readable version of the address.
    pub vault_address: VaultAddress,

    /// The amount of the resource in this vault.
    pub amount: Decimal192,

    /// The most recent state version underlying object was modified at.
    pub last_updated_at_state_version: u64,
}

impl FungibleResourcesCollectionItemVaultAggregatedVaultItem {
    pub fn new(vault_address: VaultAddress, amount: Decimal192) -> Self {
        Self {
            vault_address,
            amount,
            last_updated_at_state_version: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_decoding() {
        let json = format!(
            r#"{{
            "vault_address": "{}",
            "amount": "12.5",
            "last_updated_at_state_version": 42
        }}"#,
            VaultAddress::sample()
        );
        let item: FungibleResourcesCollectionItemVaultAggregatedVaultItem =
            serde_json::from_str(&json).unwrap();
        assert_eq!(item.amount, "12.5".parse::<Decimal192>().unwrap());
        assert_eq!(item.last_updated_at_state_version, 42);
    }
}
