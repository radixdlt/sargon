use sargon_core_time_utils::now;

use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
)]
#[display("ID {} at {}", id, generated_at)]
pub struct HostId {
    /// A best effort stable and unique identifier of this
    /// host's device.
    pub id: DeviceID,

    /// The date this id of the device was generated, might
    /// be equal to when the app was first ever launched on the
    /// device.
    pub generated_at: Timestamp,
}

impl HostId {
    pub fn generate_new() -> Self {
        Self {
            id: DeviceID::generate_new(),
            generated_at: now(),
        }
    }
}

impl HasSampleValues for HostId {
    fn sample() -> Self {
        Self {
            id: DeviceID::sample(),
            generated_at: Timestamp::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            id: DeviceID::sample_other(),
            generated_at: Timestamp::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HostId;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn test_uniqueness() {
        let host_infos: Vec<HostId> =
            (0..100).map(|_| SUT::generate_new()).collect();

        assert_eq!(
            HashSet::<SUT>::from_iter(host_infos.iter().cloned()).len(),
            100
        );
    }

    #[test]
    fn test_to_string() {
        let info = SUT::sample();
        assert_eq!(
            "ID ffffffff-ffff-ffff-ffff-ffffffffffff at 2023-09-11T16:05:56.000Z",
            info.to_string()
        )
    }
}
