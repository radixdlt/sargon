use crate::prelude::*;

impl Header {
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
    pub fn update_header(
        &mut self,
        maybe_device_info: impl Into<Option<DeviceInfo>>,
    ) {
        let content_hint = self.networks.content_hint();
        self.header.update(content_hint, maybe_device_info)
    }
}
