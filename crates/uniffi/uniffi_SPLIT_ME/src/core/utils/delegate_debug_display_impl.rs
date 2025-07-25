use crate::prelude::*;

macro_rules! delegate_display_debug_into {
    ($external_type:ty, $internal_type:ty) => {
        delegate_display_into!($external_type, $internal_type);

        delegate_debug_into!($external_type, $internal_type);
    };
}

macro_rules! delegate_display_into {
    ($external_type:ty, $internal_type:ty) => {
        impl std::fmt::Display for $external_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let internal = self.clone().into_internal();
                write!(f, "{}", internal)
            }
        }
    };
}

macro_rules! delegate_debug_into {
    ($external_type:ty, $internal_type:ty) => {
        impl std::fmt::Debug for $external_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let internal = self.clone().into_internal();
                write!(f, "{:?}", internal)
            }
        }
    };
}

pub(crate) use delegate_debug_into;
pub(crate) use delegate_display_debug_into;
pub(crate) use delegate_display_into;
