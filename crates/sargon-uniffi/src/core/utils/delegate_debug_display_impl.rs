use crate::prelude::*;

macro_rules! delegate_display_debug_into {
    ($external_type:ty, $internal_type:ty) => {
        impl std::fmt::Display for $external_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let internal = self.into_internal();
                write!(f, "{}", internal)
            }
        }

        impl std::fmt::Debug for $external_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let internal = self.into_internal();
                write!(f, "{:?}", internal)
            }
        }
    };
}

pub(crate) use delegate_display_debug_into;
