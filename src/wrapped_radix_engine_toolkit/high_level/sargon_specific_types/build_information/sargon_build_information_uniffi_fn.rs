use std::sync::Once;

use crate::prelude::*;

#[uniffi::export]
pub fn new_sargon_build_information_sample() -> SargonBuildInformation {
    SargonBuildInformation::sample()
}

#[uniffi::export]
pub fn new_sargon_build_information_sample_other() -> SargonBuildInformation {
    SargonBuildInformation::sample_other()
}

#[uniffi::export]
pub fn enable_logging_from_rust() {
    init_logging()
}

/// Initializes logging
#[cfg(not(tarpaulin_include))] // actually tricky, since we init logging from other unit tests -> crash.
pub fn init_logging() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let level = log::LevelFilter::Trace;
        pretty_env_logger::formatted_builder()
            .filter_level(level)
            .try_init()
            .expect("Should be able to setup a logger.");
        info!("Rust: Logger initialized, log level: {:?}", level);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonBuildInformation;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_sargon_build_information_sample(),
                new_sargon_build_information_sample_other(),
                // duplicates should get removed
                new_sargon_build_information_sample(),
                new_sargon_build_information_sample_other(),
            ])
            .len(),
            2
        );
    }
}
