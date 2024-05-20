use crate::prelude::*;

#[uniffi::export]
pub fn new_network_request_sample() -> NetworkRequest {
    NetworkRequest::sample()
}

#[uniffi::export]
pub fn new_network_request_sample_other() -> NetworkRequest {
    NetworkRequest::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NetworkRequest;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_network_request_sample(),
                new_network_request_sample_other(),
                // duplicates should get removed
                new_network_request_sample(),
                new_network_request_sample_other(),
            ])
            .len(),
            2
        );
    }
}
