use crate::prelude::*;

#[uniffi::export]
pub fn new_non_fungible_global_id_from_string(
    global_id: String,
) -> Result<NonFungibleGlobalId> {
    NonFungibleGlobalId::from_str(&global_id)
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleGlobalId;

    #[test]
    fn test_from_global_id() {
        let global_id = SUT::new(
            NonFungibleResourceAddress::sample_mainnet(),
            NonFungibleLocalId::integer(1),
        );

        assert_eq!(
            new_non_fungible_global_id_from_string(global_id.to_string())
                .unwrap(),
            global_id
        );
    }
}
