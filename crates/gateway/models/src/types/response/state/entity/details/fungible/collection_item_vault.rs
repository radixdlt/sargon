use crate::prelude::*;

/// A fungible resource an entity holds, listed per vault.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct FungibleResourcesCollectionItemVaultAggregated {
    pub resource_address: ResourceAddress,

    pub vaults: FungibleResourcesCollectionItemVaults,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct FungibleResourcesCollectionItemVaults {
    pub items: Vec<FungibleResourcesCollectionItemVaultAggregatedVaultItem>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct FungibleResourcesCollectionItemVaultAggregatedVaultItem {
    pub vault_address: VaultAddress,

    pub amount: Decimal192,
}

impl FungibleResourcesCollectionItemVaultAggregatedVaultItem {
    pub fn new(vault_address: VaultAddress, amount: Decimal192) -> Self {
        Self {
            vault_address,
            amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_decoding_of_a_fungible_vaults_page_item() {
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
        assert_eq!(
            item,
            FungibleResourcesCollectionItemVaultAggregatedVaultItem::new(
                VaultAddress::sample(),
                "12.5".parse::<Decimal192>().unwrap()
            )
        );
    }
}
