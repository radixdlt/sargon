use crate::prelude::*;
use sargon::AccessRequirementForDisplay as InternalAccessRequirementForDisplay;
use sargon::BadgeForDisplay as InternalBadgeForDisplay;

/// A requirement of an access rule as shown to the user: simplified, with each
/// badge resolved to what it is, e.g. one of the user's factors.
///
/// Its shape follows from the engine's rules, so hosts only draw:
/// * `Anyone` and `NoOne` for a whole role, never nested;
/// * `AnyOf` and `AllOf` with at least two requirements, alternating: an
///   `AnyOf` never directly holds an `AnyOf`, nor an `AllOf` an `AllOf`;
/// * `CountOf` with only badges, and a count above one and below their
///   number.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum AccessRequirementForDisplay {
    /// Passes without any proof.
    Anyone,

    /// Never passes.
    NoOne,

    /// A proof of this badge.
    Badge { badge: BadgeForDisplay },

    /// A proof of at least `amount` of `resource`.
    AmountOf {
        amount: Decimal192,
        resource: ResourceAddress,
    },

    /// At least one of `requirements` must pass.
    AnyOf {
        requirements: Vec<AccessRequirementForDisplay>,
    },

    /// All of `requirements` must pass.
    AllOf {
        requirements: Vec<AccessRequirementForDisplay>,
    },

    /// At least `count` of `badges` must be proven.
    CountOf {
        count: u32,
        badges: Vec<BadgeForDisplay>,
    },
}

/// What a badge is, as far as the wallet knows.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum BadgeForDisplay {
    /// A signature with one of the user's factors.
    OwnFactor { factor_source: FactorSource },

    /// A signature with a key the wallet doesn't know, e.g. a co-signer's or a
    /// dApp's.
    UnknownKey { badge: NonFungibleGlobalId },

    /// A non-fungible that isn't a signature, e.g. an NFT badge.
    NonFungible { badge: NonFungibleGlobalId },

    /// Any amount of a resource.
    Resource { resource: ResourceAddress },
}

#[uniffi::export]
pub fn new_access_requirement_for_display_sample() -> AccessRequirementForDisplay
{
    InternalAccessRequirementForDisplay::sample().into()
}

#[uniffi::export]
pub fn new_access_requirement_for_display_sample_other(
) -> AccessRequirementForDisplay {
    InternalAccessRequirementForDisplay::sample_other().into()
}

decl_conversion_tests_for!(AccessRequirementForDisplay);
decl_conversion_tests_for!(BadgeForDisplay);
