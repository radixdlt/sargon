use crate::prelude::*;
use sargon::ResourceOrNonFungible as InternalResourceOrNonFungible;

decl_vec_samples_for!(DepositorsAllowList, ResourceOrNonFungible);

/// The addresses that can be added as exception to the `DepositRule`
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum ResourceOrNonFungible {
    Resource { value: ResourceAddress },

    NonFungible { value: NonFungibleGlobalId },
}

#[uniffi::export]
pub fn new_resource_or_non_fungible_sample() -> ResourceOrNonFungible {
    InternalResourceOrNonFungible::sample().into()
}

#[uniffi::export]
pub fn new_resource_or_non_fungible_sample_other() -> ResourceOrNonFungible {
    InternalResourceOrNonFungible::sample_other().into()
}

decl_conversion_tests_for!(ResourceOrNonFungible);
