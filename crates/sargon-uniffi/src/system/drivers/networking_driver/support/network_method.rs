use crate::prelude::*;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    strum::EnumString,
    strum::Display,
    uniffi::Enum,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum NetworkMethod {
    Post,
    Get,
    Head,
}

#[uniffi::export]
pub fn new_network_method_sample() -> NetworkMethod {
    NetworkMethod::sample()
}

#[uniffi::export]
pub fn new_network_method_sample_other() -> NetworkMethod {
    NetworkMethod::sample_other()
}

#[uniffi::export]
pub fn network_method_to_string(method: &NetworkMethod) -> String {
    method.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NetworkMethod;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_network_method_sample(),
                new_network_method_sample_other(),
                // duplicates should get removed
                new_network_method_sample(),
                new_network_method_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_network_method_to_string() {
        let sut = SUT::Post;
        assert_eq!(network_method_to_string(&sut), sut.to_string());
    }
}
