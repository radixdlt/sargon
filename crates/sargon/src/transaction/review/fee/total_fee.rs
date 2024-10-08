use crate::prelude::*;

/// Represents the total fee with minimum and maximum values.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct TotalFee {
    pub min: Decimal192,
    pub max: Decimal192,
}

impl TotalFee {
    /// Creates a new `TotalFee` with the given minimum and maximum values.
    pub fn new(min: Decimal192, max: Decimal192) -> Self {
        Self { min, max }
    }

    /// Returns the lock fee, which is always the maximum amount.
    pub fn lock_fee(&self) -> Decimal192 {
        // We always lock the max amount
        self.max
    }

    /// Returns a formatted string representing the total fee.
    /// If the maximum fee is greater than the minimum fee, it shows a range.
    /// Otherwise, it shows the minimum fee.
    pub fn displayed_total_fee(
        &self,
        format: impl Fn(Decimal192) -> String,
    ) -> String {
        if self.max > self.min {
            format!("{} - {}", format(self.min), format(self.max))
        } else {
            format!("{}", format(self.min))
        }
    }
}
