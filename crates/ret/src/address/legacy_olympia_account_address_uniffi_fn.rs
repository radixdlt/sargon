use crate::prelude::*;

#[uniffi::export]
pub fn new_legacy_olympia_account_address_sample() -> LegacyOlympiaAccountAddress
{
    LegacyOlympiaAccountAddress::sample()
}

#[uniffi::export]
pub fn new_legacy_olympia_account_address_sample_other(
) -> LegacyOlympiaAccountAddress {
    LegacyOlympiaAccountAddress::sample_other()
}

#[uniffi::export]
pub fn new_legacy_olympia_account_address_from_public_key(
    public_key: Secp256k1PublicKey,
) -> LegacyOlympiaAccountAddress {
    LegacyOlympiaAccountAddress::from(public_key)
}

#[uniffi::export]
pub fn new_legacy_olympia_account_address_from_string(
    string: String,
) -> Result<LegacyOlympiaAccountAddress> {
    LegacyOlympiaAccountAddress::from_str(&string)
}

#[uniffi::export]
pub fn legacy_olympia_account_address_to_string(
    address: &LegacyOlympiaAccountAddress,
) -> String {
    address.to_string()
}

#[uniffi::export]
pub fn legacy_olympia_account_address_formatted(
    address: &LegacyOlympiaAccountAddress,
    format: AddressFormat,
) -> String {
    address.formatted(format)
}

#[uniffi::export]
pub fn legacy_olympia_account_address_to_babylon_account_address(
    address: LegacyOlympiaAccountAddress,
) -> AccountAddress {
    address.to_babylon_account_address()
}

#[uniffi::export]
pub fn legacy_olympia_account_address_is_legacy_of_babylon(
    legacy_olympia_address: &LegacyOlympiaAccountAddress,
    babylon_account_address: &AccountAddress,
) -> bool {
    babylon_account_address.was_migrated_from_legacy_olympia_account_address(
        legacy_olympia_address,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let s =
            "rdx1qsp8n0nx0muaewav2ksx99wwsu9swq5mlndjmn3gm9vl9q2mzmup0xqm2ylge";
        assert_eq!(
            new_legacy_olympia_account_address_from_string(s.to_owned())
                .unwrap(),
            SUT::sample_other()
        );
    }

    #[test]
    fn test_formatted() {
        assert_eq!(
            legacy_olympia_account_address_formatted(
                &SUT::sample_other(),
                AddressFormat::Default
            ),
            "rdx...0xqm2ylge"
        );
    }

    #[test]
    fn test_legacy_olympia_account_address_to_string() {
        assert_eq!(
            legacy_olympia_account_address_to_string(&SUT::sample_other()),
            "rdx1qsp8n0nx0muaewav2ksx99wwsu9swq5mlndjmn3gm9vl9q2mzmup0xqm2ylge"
        );
    }

    #[test]
    fn test_new_legacy_olympia_account_address_from_public_key() {
        let public_key: Secp256k1PublicKey = "026f08db98ef1d0231eb15580da9123db8e25aa1747c8c32e5fd2ec47b8db73d5c".parse().unwrap();
        let sut =
            new_legacy_olympia_account_address_from_public_key(public_key);
        assert_eq!(sut, SUT::sample());
    }

    #[test]
    fn test_legacy_olympia_account_address_to_babylon_account_address() {
        assert_eq!(
            legacy_olympia_account_address_to_babylon_account_address(SUT::sample()).to_string(),
            "account_rdx168e8u653alt59xm8ple6khu6cgce9cfx9mlza6wxf7qs3wwdh0pwrf"
        )
    }

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LegacyOlympiaAccountAddress;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_legacy_olympia_account_address_sample(),
                new_legacy_olympia_account_address_sample_other(),
                // duplicates should get removed
                new_legacy_olympia_account_address_sample(),
                new_legacy_olympia_account_address_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_legacy_olympia_account_address_is_legacy_of_babylon() {
        let babylon: AccountAddress = "account_rdx168e8u653alt59xm8ple6khu6cgce9cfx9mlza6wxf7qs3wwdh0pwrf".parse().unwrap();
        assert!(legacy_olympia_account_address_is_legacy_of_babylon(
            &SUT::sample(),
            &babylon
        ));
    }
}
