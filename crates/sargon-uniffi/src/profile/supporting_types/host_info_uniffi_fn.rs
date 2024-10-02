use crate::prelude::*;

#[uniffi::export]
pub fn new_host_info_sample() -> HostInfo {
    HostInfo::sample()
}

#[uniffi::export]
pub fn new_host_info_sample_other() -> HostInfo {
    HostInfo::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HostInfo;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_host_info_sample(),
                new_host_info_sample_other(),
                // duplicates should get removed
                new_host_info_sample(),
                new_host_info_sample_other(),
            ])
            .len(),
            2
        );
    }
}
