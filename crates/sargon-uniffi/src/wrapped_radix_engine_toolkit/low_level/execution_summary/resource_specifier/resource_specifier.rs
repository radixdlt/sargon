use crate::prelude::*;
use sargon::ResourceSpecifier as InternalResourceSpecifier;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum ResourceSpecifier {
    Fungible {
        resource_address: ResourceAddress,
        amount: Decimal,
    },
    NonFungible {
        resource_address: ResourceAddress,
        ids: Vec<NonFungibleLocalId>,
    },
}

impl From<InternalResourceSpecifier> for ResourceSpecifier {
    fn from(value: InternalResourceSpecifier) -> Self {
        match value {
            InternalResourceSpecifier::Fungible {
                resource_address,
                amount,
            } => ResourceSpecifier::Fungible {
                resource_address: resource_address.into(),
                amount: amount.into(),
            },
            InternalResourceSpecifier::NonFungible { resource_address, ids } => {
                ResourceSpecifier::NonFungible {
                    resource_address: resource_address.into(),
                    ids: ids.into_iter().map(Into::into).collect(),
                }
            }
        }
    }
}

impl Into<InternalResourceSpecifier> for ResourceSpecifier {
    fn into(self) -> InternalResourceSpecifier {
        match self {
            ResourceSpecifier::Fungible {
                resource_address,
                amount,
            } => InternalResourceSpecifier::Fungible {
                resource_address: resource_address.into(),
                amount: amount.into(),
            },
            ResourceSpecifier::NonFungible {
                resource_address,
                ids,
            } => InternalResourceSpecifier::NonFungible {
                resource_address: resource_address.into(),
                ids: ids.into_iter().map(Into::into).collect(),
            },
        }
    }
}

#[uniffi::export]
pub fn new_resource_specifier_sample() -> ResourceSpecifier {
    InternalResourceSpecifier::sample().into()
}

#[uniffi::export]
pub fn new_resource_specifier_sample_other() -> ResourceSpecifier {
    InternalResourceSpecifier::sample_other().into()
}

#[uniffi::export]
pub fn resource_specifier_get_address(
    specifier: &ResourceSpecifier,
) -> ResourceAddress {
    specifier.into::<InternalResourceSpecifier>().get_address().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceSpecifier;

    #[test]
    fn inequality() {
        assert_ne!(
            new_resource_indicator_sample(),
            new_resource_indicator_sample_other()
        );
    }

    #[test]
    fn get_address() {
        let sut = SUT::sample();
        assert_eq!(sut.get_address(), resource_specifier_get_address(&sut));
    }
}
