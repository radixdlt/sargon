use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum FeeMode {
    Normal {
        customization: NormalFeeCustomization,
    },
    Advanced {
        customization: AdvancedFeeCustomization,
    },
}
