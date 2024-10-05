use crate::prelude::*;

/// A stable and globally unique identifier of a device,
/// e.g. an Android phone.
#[derive(
    Debug,
    Copy,
    derive_more::Display,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]

pub struct DeviceID(pub(crate) Uuid);
uniffi::custom_newtype!(DeviceID, Uuid);