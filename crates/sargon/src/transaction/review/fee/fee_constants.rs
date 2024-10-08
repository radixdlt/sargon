use crate::prelude::*;

/// Network fees -> https://radixdlt.atlassian.net/wiki/spaces/S/pages/3134783512/Manifest+Mutation+Cost+Addition+Estimates
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeeConstants;

impl FeeConstants {
    const NETWORK_FEE_MULTIPLIER: f32 = 0.15;
    const LOCK_FEE_INSTRUCTION_COST: f64 = 0.08581566997;
    const SIGNATURE_COST: f64 = 0.01109974758;
    const NOTARIZING_COST: f64 = 0.0081393944;
    const NOTARIZING_COST_WHEN_NOTARY_IS_SIGNATORY: f64 = 0.0084273944;
    const FUNGIBLE_GUARANTEE_INSTRUCTION_COST: f64 = 0.00908532837;

    /// Returns the network fee multiplier as a `Decimal192`.
    pub fn network_fee_multiplier() -> Decimal192 {
        Self::NETWORK_FEE_MULTIPLIER.into()
    }

    /// Returns the lock fee instruction cost as a `Decimal192`.
    pub fn lock_fee_instruction_cost() -> Decimal192 {
        Self::convert(Self::LOCK_FEE_INSTRUCTION_COST)
    }

    /// Returns the fungible guarantee instruction cost as a `Decimal192`.
    pub fn fungible_guarantee_instruction_cost() -> Decimal192 {
        Self::convert(Self::FUNGIBLE_GUARANTEE_INSTRUCTION_COST)
    }

    /// Returns the lock fee cost based on whether the lock fee is included.
    pub fn lock_fee_cost(include_lock_fee: bool) -> Decimal192 {
        if include_lock_fee {
            Self::convert(Self::LOCK_FEE_INSTRUCTION_COST)
        } else {
            Decimal192::zero()
        }
    }

    /// Returns the notarizing cost based on whether the notary is a signatory.
    pub fn notarizing_cost(notary_is_signatory: bool) -> Decimal192 {
        Self::convert(if notary_is_signatory {
            Self::NOTARIZING_COST_WHEN_NOTARY_IS_SIGNATORY
        } else {
            Self::NOTARIZING_COST
        })
    }

    /// Returns the total signature cost based on the number of signatures.
    pub fn signatures_cost(signature_count: usize) -> Decimal192 {
        Decimal192::from(signature_count as i64)
            * Self::convert(Self::SIGNATURE_COST)
    }

    fn convert(value: f64) -> Decimal192 {
        value
            .try_into()
            .expect("Should be able to convert f64 to Decimal192")
    }
}
