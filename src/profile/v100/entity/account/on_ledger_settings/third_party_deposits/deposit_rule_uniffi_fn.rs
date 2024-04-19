use crate::prelude::*;

#[uniffi::export]
pub fn new_deposit_rule_sample() -> DepositRule {
    DepositRule::sample()
}

#[uniffi::export]
pub fn new_deposit_rule_sample_other() -> DepositRule {
    DepositRule::sample_other()
}

#[uniffi::export]
pub fn new_deposit_rule_from_json_string(
    json_string: String,
) -> Result<DepositRule> {
    DepositRule::from_json_string(json_string)
}

#[uniffi::export]
pub fn deposit_rule_to_json_string(rule: &DepositRule) -> String {
    rule.to_json_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DepositRule;

    #[test]
    fn json_string_roundtrip() {
        let sut = SUT::sample();
        let json = deposit_rule_to_json_string(&sut);
        assert_eq!(json, "acceptKnown");
        assert_eq!(sut, new_deposit_rule_from_json_string(json).unwrap())
    }

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
