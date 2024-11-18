use crate::prelude::*;

impl Account {
    /// Marks the account as hidden
    pub fn mark_as_hidden(&mut self) {
        self.flags.insert_flag(EntityFlag::HiddenByUser);
    }

    /// Marks the account as tombstoned
    pub fn mark_as_tombstoned(&mut self) {
        self.flags.insert_flag(EntityFlag::TombstonedByUser);
        self.flags.remove_flag(&EntityFlag::HiddenByUser);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Account;

    #[test]
    fn test_mark_as_hidden_is_hidden() {
        let mut sut = SUT::sample();
        sut.mark_as_hidden();
        assert!(sut.is_hidden())
    }

    #[test]
    fn test_currently_hidden_remains_hidden() {
        let mut sut = SUT::sample_mainnet_diana();
        sut.mark_as_hidden();
        assert!(sut.is_hidden())
    }

    #[test]
    fn test_mark_as_tombstoned_is_tombstoned() {
        let mut sut = SUT::sample();
        sut.mark_as_tombstoned();
        assert!(sut.is_tombstoned())
    }

    #[test]
    fn test_currently_tombstoned_remains_tombstoned() {
        let mut sut = SUT::sample_mainnet_erin();
        sut.mark_as_tombstoned();
        assert!(sut.is_tombstoned())
    }

    #[test]
    fn test_currently_hidden_when_tombstoned_not_hidden() {
        let mut sut = SUT::sample_mainnet_diana();
        sut.mark_as_tombstoned();
        assert!(sut.is_tombstoned());
        assert!(!sut.is_hidden())
    }
}
