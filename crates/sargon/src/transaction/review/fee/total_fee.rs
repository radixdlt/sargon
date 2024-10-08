use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct TotalFee {
    pub min: Decimal192,
    pub max: Decimal192,
}

impl TotalFee {
    pub fn new(min: Decimal192, max: Decimal192) -> Self {
        Self { min, max }
    }

    pub fn lock_fee(&self) -> Decimal192 {
        // We always lock the max amount
        self.max
    }

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
