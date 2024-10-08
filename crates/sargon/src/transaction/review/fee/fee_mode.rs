use crate::prelude::*;

/// Represents the fee mode for a transaction.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum FeeMode {
    /// Normal fee mode with standard customization.
    Normal {
        customization: NormalFeeCustomization,
    },
    /// Advanced fee mode with additional customization options.
    Advanced {
        customization: AdvancedFeeCustomization,
    },
}
