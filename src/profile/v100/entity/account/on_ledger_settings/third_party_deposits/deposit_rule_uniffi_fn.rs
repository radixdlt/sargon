use crate::prelude::*;

json_string_convertible!(DepositRule, "super invalid json string");

#[uniffi::export]
pub fn new_deposit_rule_sample() -> DepositRule {
    DepositRule::sample()
}

#[uniffi::export]
pub fn new_deposit_rule_sample_other() -> DepositRule {
    DepositRule::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DepositRule;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_deposit_rule_sample(),
                new_deposit_rule_sample_other(),
                // duplicates should get removed
                new_deposit_rule_sample(),
                new_deposit_rule_sample_other(),
            ])
            .len(),
            2
        );
    }
}
