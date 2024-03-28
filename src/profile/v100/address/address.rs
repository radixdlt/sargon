use crate::{address_union, prelude::*};

address_union!(
    /// A tagged union of addresses.
    ///
    /// Does not include `LegacyOlympiaAccountAddress` nor `NonFungibleResourceAddress`
    enum Address: accessController, account, component, identity, package, pool, resource, validator, vault
);

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Address;

    #[test]
    fn sample_values_count() {
        let mut set = HashSet::<SUT>::new();
        set.extend(SUT::sample_values_all());
        // Duplicates should be removed
        set.extend(SUT::sample_values_all());

        assert_eq!(set.len(), 36);
    }
}
