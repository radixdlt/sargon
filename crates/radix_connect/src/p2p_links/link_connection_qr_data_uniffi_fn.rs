use crate::prelude::*;

json_data_convertible!(LinkConnectionQRData);

#[uniffi::export]
pub fn new_link_connection_qr_data_sample() -> LinkConnectionQRData {
    LinkConnectionQRData::sample()
}

#[uniffi::export]
pub fn new_link_connection_qr_data_sample_other() -> LinkConnectionQRData {
    LinkConnectionQRData::sample_other()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LinkConnectionQRData;

    #[test]
    fn sample_values() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_link_connection_qr_data_sample(),
                new_link_connection_qr_data_sample_other(),
                // duplicates should get removed
                new_link_connection_qr_data_sample(),
                new_link_connection_qr_data_sample_other(),
            ])
            .len(),
            2
        );
    }
}
