use crate::{address_union, prelude::*};

address_union!(
    /// A tagged union of addresses.
    ///
    /// Does not include `LegacyOlympiaAccountAddress` nor `NonFungibleResourceAddress`
    enum Address: accessController, account, component, identity, package, pool, resource, validator, vault, locker
);
