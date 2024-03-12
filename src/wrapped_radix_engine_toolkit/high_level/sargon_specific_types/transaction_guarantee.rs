use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct TransactionGuarantee {
    amount: Decimal192,
    pub instruction_index: u64,
    pub resource_address: ResourceAddress,
    pub resource_divisibility: Option<i32>,
}

impl TransactionGuarantee {
    pub fn new(
        amount: impl Into<Decimal192>,
        instruction_index: u64,
        resource_address: ResourceAddress,
        resource_divisibility: impl Into<Option<i32>>,
    ) -> Self {
        Self {
            amount: amount.into(),
            instruction_index,
            resource_address,
            resource_divisibility: resource_divisibility.into(),
        }
    }
}

impl TransactionGuarantee {
    pub(crate) fn rounded_amount(&self) -> Decimal192 {
        self.amount.clone().round(self.resource_divisibility)
    }
}

impl HasSampleValues for TransactionGuarantee {
    fn sample() -> Self {
        TransactionGuarantee::new(1337, 3, ResourceAddress::sample(), Some(12))
    }

    fn sample_other() -> Self {
        TransactionGuarantee::new(42, 12, ResourceAddress::sample_other(), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionGuarantee;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn rounding() {
        let sut = SUT::new(
            "0.12344".parse::<Decimal192>().unwrap(),
            2,
            ResourceAddress::sample_mainnet_candy(),
            4,
        );

        assert_eq!(
            sut.rounded_amount(),
            "0.1234".parse::<Decimal192>().unwrap()
        );
    }
}
