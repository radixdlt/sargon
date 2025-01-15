use crate::prelude::*;

/// Account or Identity (used by Personas) part of a CAP26 derivation
/// path.
#[derive(
    Serialize_repr,
    Deserialize_repr,
    FromRepr,
    Clone,
    Copy,
    EnumAsInner,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::Debug,
)]
#[repr(u32)]
pub enum CAP26EntityKind {
    /// An Account entity type
    #[display("Account")]
    Account = 525,

    /// An Identity entity type (used by Personas)
    #[display("Identity")]
    Identity = 618,
}

impl CAP26EntityKind {
    /// The raw representation of this entity kind, a `u32`.
    pub fn discriminant(&self) -> u32 {
        *self as u32
    }
}

impl TryFrom<U31> for CAP26EntityKind {
    type Error = CommonError;
    fn try_from(value: U31) -> Result<Self> {
        let repr = u32::from(value);
        Self::from_repr(repr)
            .ok_or(CommonError::InvalidEntityKind { bad_value: repr })
    }
}

impl HasSampleValues for CAP26EntityKind {
    fn sample() -> Self {
        CAP26EntityKind::Account
    }

    fn sample_other() -> Self {
        CAP26EntityKind::Identity
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = CAP26EntityKind;

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
    fn discriminant() {
        assert_eq!(SUT::Account.discriminant(), 525);
        assert_eq!(SUT::Identity.discriminant(), 618);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::Account), "Account");
        assert_eq!(format!("{}", SUT::Identity), "Identity");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", SUT::Account), "Account");
        assert_eq!(format!("{:?}", SUT::Identity), "Identity");
    }

    #[test]
    fn hash() {
        assert_eq!(
            BTreeSet::from_iter([SUT::Account, SUT::Account].into_iter()).len(),
            1
        );
    }

    #[test]
    fn ord() {
        assert!(SUT::Account < SUT::Identity);
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(&SUT::Account, json!(525));
        assert_json_roundtrip(&SUT::Account);
    }
}
