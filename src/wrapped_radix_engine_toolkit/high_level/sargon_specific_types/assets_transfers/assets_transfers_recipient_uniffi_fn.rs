use crate::prelude::*;

#[uniffi::export]
pub fn new_assets_transfers_recipient_sample() -> AssetsTransfersRecipient {
    AssetsTransfersRecipient::sample()
}

#[uniffi::export]
pub fn new_assets_transfers_recipient_sample_other() -> AssetsTransfersRecipient
{
    AssetsTransfersRecipient::sample_other()
}

#[uniffi::export]
pub fn assets_transfers_recipient_account_address(
    recipient: &AssetsTransfersRecipient,
) -> AccountAddress {
    *recipient.account_address()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AssetsTransfersRecipient;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_assets_transfers_recipient_sample(),
                new_assets_transfers_recipient_sample_other(),
                // duplicates should get removed
                new_assets_transfers_recipient_sample(),
                new_assets_transfers_recipient_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_account_address() {
        let sut = SUT::sample();
        assert_eq!(
            assets_transfers_recipient_account_address(&sut),
            *sut.account_address()
        )
    }
}
