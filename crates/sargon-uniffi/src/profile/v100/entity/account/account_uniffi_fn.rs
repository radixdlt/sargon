use crate::prelude::*;

#[uniffi::export]
pub fn new_account_sample_mainnet_alice() -> Account {
    Account::sample_mainnet_alice()
}

#[uniffi::export]
pub fn new_account_sample_mainnet_bob() -> Account {
    Account::sample_mainnet_bob()
}

#[uniffi::export]
pub fn new_account_sample_mainnet_carol() -> Account {
    Account::sample_mainnet_carol()
}

#[uniffi::export]
pub fn new_account_sample_mainnet_diana() -> Account {
    Account::sample_mainnet_diana()
}

#[uniffi::export]
pub fn new_account_sample_stokenet_nadia() -> Account {
    Account::sample_stokenet_nadia()
}

#[uniffi::export]
pub fn new_account_sample_stokenet_olivia() -> Account {
    Account::sample_stokenet_olivia()
}

#[uniffi::export]
pub fn new_account_sample_stokenet_paige() -> Account {
    Account::sample_stokenet_paige()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Account;

    #[test]
    fn hash_of_sample_values() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_account_sample_mainnet_alice(),
                new_account_sample_mainnet_bob(),
                new_account_sample_mainnet_carol(),
                new_account_sample_mainnet_diana(),
                new_account_sample_stokenet_nadia(),
                new_account_sample_stokenet_olivia(),
                new_account_sample_stokenet_paige(),
                // duplicates should be removed
                new_account_sample_mainnet_alice(),
                new_account_sample_mainnet_bob(),
                new_account_sample_mainnet_carol(),
                new_account_sample_mainnet_diana(),
                new_account_sample_stokenet_nadia(),
                new_account_sample_stokenet_olivia(),
                new_account_sample_stokenet_paige(),
            ])
            .len(),
            7
        )
    }
}
