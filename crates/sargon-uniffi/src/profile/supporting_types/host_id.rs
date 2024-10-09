use crate::prelude::*;
use sargon::HostId as InternalHostId;

#[derive(
    Clone,
    
    Debug,
    PartialEq,
    Eq,
    Hash,
     uniffi::Record,
)]
pub struct HostId {
    /// A best effort stable and unique identifier of this
    /// host's device.
    pub id: DeviceID,

    /// The date this id of the device was generated, might
    /// be equal to when the app was first ever launched on the
    /// device.
    pub generated_at: Timestamp,
}

impl From<InternalHostId> for HostId {
    fn from(value: InternalHostId) -> Self {
        Self {
            id: value.id.into(),
            generated_at: value.generated_at.into(),
        }
    }
}

impl Into<InternalHostId> for HostId {
    fn into(self) -> InternalHostId {
        InternalHostId {
            id: self.id.into(),
            generated_at: self.generated_at.into(),
        }
    }
}

json_data_convertible!(HostId);

#[uniffi::export]
pub fn new_host_id_sample() -> HostId {
    InternalHostId::sample().into()
}

#[uniffi::export]
pub fn new_host_id_sample_other() -> HostId {
    InternalHostId::sample_other().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HostId;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_host_id_sample(),
                new_host_id_sample_other(),
                // duplicates should get removed
                new_host_id_sample(),
                new_host_id_sample_other(),
            ])
            .len(),
            2
        );
    }
}
