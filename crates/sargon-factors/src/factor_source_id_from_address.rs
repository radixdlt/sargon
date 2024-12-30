use crate::prelude::*;

/// FactorSourceID from an AccountAddress, typically used by `trustedContact` FactorSource.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
)]
#[display("{}", self.to_canonical_string())]
#[debug("{}", self.to_canonical_string())]
pub struct FactorSourceIDFromAddress {
    /// The kind of the FactorSource this ID refers to, typically `trustedContact`.
    pub kind: FactorSourceKind,

    /// An account address which the FactorSource this ID refers uses/needs.
    pub body: ShortString, // actually `AccountAddress` - TODO split out addresses into separate crate and add it dependency in this crate...?
}

impl FactorSourceIDFromAddress {
    pub fn new(kind: FactorSourceKind, body: impl AsRef<str>) -> Self {
        assert!(kind == FactorSourceKind::TrustedContact, "Only supported FactorSourceKind to be used with  FactorSourceIDFromAddress is `trustedContact` at this moment.");
        Self {
            kind,
            body: ShortString::new(body).unwrap(),
        }
    }
}

impl FactorSourceIDFromAddress {
    pub fn new_for_trusted_contact(address: impl AsRef<str>) -> Self {
        Self::new(FactorSourceKind::TrustedContact, address.as_ref())
    }
}

impl FactorSourceIDFromAddress {
    pub fn to_canonical_string(&self) -> String {
        format!("{}:{}", self.kind.discriminant(), self.body)
    }
}

impl FactorSourceIDFromAddress {
    /// A sample used to facilitate unit tests.
    pub fn sample_trusted_contact_friend_frank() -> Self {
        Self::new_for_trusted_contact("account_rdx1298d59ae3k94htjzpy2z6mx4436h98e5u4qpnwhek8lukv7lkfrank")
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_trusted_contact_friend_judy() -> Self {
        Self::new_for_trusted_contact("account_rdx12y0389ew2xn7w02d059hhye6t0mjzqxqyavsetyg2j3p3xqyepjudy")
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_trusted_contact_friend_oscar() -> Self {
        Self::new_for_trusted_contact("account_rdx129uc6rf5vmkj2gau7fgxlsqdg8008nca8yd57sxx4v67dyw7u0scar")
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_trusted_entity_radix() -> Self {
        Self::new_for_trusted_contact("account_rdx12y7uww27s250g9d3d72ey9wdp5z78zpmq5la0r0wgw4fkf6y8eerdx")
    }
}

impl HasSampleValues for FactorSourceIDFromAddress {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_trusted_contact_friend_frank()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::sample_trusted_contact_friend_oscar()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            FactorSourceIDFromAddress::sample(),
            FactorSourceIDFromAddress::sample()
        );
        assert_eq!(
            FactorSourceIDFromAddress::sample_other(),
            FactorSourceIDFromAddress::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            FactorSourceIDFromAddress::sample(),
            FactorSourceIDFromAddress::sample_other()
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", FactorSourceIDFromAddress::sample()),
            "trustedContact:account_rdx1298d59ae3k94htjzpy2z6mx4436h98e5u4qpnwhek8lukv7lkfrank"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", FactorSourceIDFromAddress::sample()),
            "trustedContact:account_rdx1298d59ae3k94htjzpy2z6mx4436h98e5u4qpnwhek8lukv7lkfrank"
        );
    }

    #[test]
    fn json_roundtrip() {
        let model = FactorSourceIDFromAddress::sample();

        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "kind": "trustedContact",
                "body": "account_rdx1298d59ae3k94htjzpy2z6mx4436h98e5u4qpnwhek8lukv7lkfrank"
            }
            "#,
        );
    }
}
