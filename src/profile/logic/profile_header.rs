use crate::prelude::*;

impl Header {
    /// Updates `last_modified`, `content_hint` and also `last_used_on_device` if
    /// it was specified.
    pub fn update(
        &mut self,
        content_hint: ContentHint,
        maybe_device_info: impl Into<Option<DeviceInfo>>,
    ) {
        if let Some(device_info) = maybe_device_info.into() {
            self.last_used_on_device = device_info;
        }
        self.content_hint = content_hint;
        self.last_modified = now();
    }
}

impl Profile {
    /// Updates the header's fields: `last_modified`, `content_hint` and also
    /// `last_used_on_device` if it was specified.
    pub fn update_header(
        &mut self,
        maybe_device_info: impl Into<Option<DeviceInfo>>,
    ) {
        let content_hint = self.networks.content_hint();
        self.header.update(content_hint, maybe_device_info)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Header;

    #[test]
    fn test_update_header() {
        let mut sut = SUT::sample();
        let new_hint = ContentHint::sample_other();
        let new_device = DeviceInfo::sample_other();
        assert_ne!(sut.content_hint, new_hint.clone());
        assert_ne!(sut.last_used_on_device, new_device.clone());
        sut.update(ContentHint::sample_other(), DeviceInfo::sample_other());
        assert_eq!(sut.content_hint, new_hint);
        assert_eq!(sut.last_used_on_device, new_device);
    }
}
