use crate::prelude::*;

json_data_convertible!(RequestedQuantity);

#[uniffi::export]
pub fn new_requested_quantity_sample() -> RequestedQuantity {
    RequestedQuantity::sample()
}

#[uniffi::export]
pub fn new_requested_quantity_sample_other() -> RequestedQuantity {
    RequestedQuantity::sample_other()
}

#[uniffi::export]
pub fn requested_quantity_is_valid(
    requested_quantity: RequestedQuantity,
) -> bool {
    requested_quantity.is_valid()
}

/// Checks `number_of_ids` can fulfill the [`RequestedQuantity`] (self), `number_of_ids` is
/// considered to be fulfilling the requested quantity:
/// * if: quantifier == ::Exactly && number_of_ids == quantity // ✅ fulfills
/// * else if: quantifier == ::AtLeast && number_of_ids >= quantity // ✅ fulfills
/// * else false // ❌ does NOT fulfill
#[uniffi::export]
pub fn requested_quantity_is_fulfilled_by_ids(
    requested_quantity: RequestedQuantity,
    number_of_ids: u64,
) -> bool {
    requested_quantity.is_fulfilled_by_ids(number_of_ids as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RequestedQuantity;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_requested_quantity_sample(),
                new_requested_quantity_sample_other(),
                // duplicates should get removed
                new_requested_quantity_sample(),
                new_requested_quantity_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_is_valid() {
        assert!(requested_quantity_is_valid(SUT::sample()))
    }

    #[test]
    fn test_is_fulfilled() {
        assert!(requested_quantity_is_fulfilled_by_ids(SUT::sample(), 1));
        assert!(!requested_quantity_is_fulfilled_by_ids(SUT::sample(), 2));
    }
}
