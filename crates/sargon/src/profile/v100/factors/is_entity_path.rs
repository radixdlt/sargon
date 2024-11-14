use crate::prelude::*;

pub trait IsEntityPath:
    NewEntityPath
    + IsNetworkAware
    + HasEntityKind
    + HasKeyKindObjectSafe
    + Clone
    + Into<DerivationPath>
    + TryFrom<DerivationPath, Error = CommonError>
{}

impl<
        T: NewEntityPath
            + Clone
            + IsNetworkAware
            + HasEntityKind
            + HasKeyKindObjectSafe
            + Into<DerivationPath>
            + TryFrom<DerivationPath, Error = CommonError>,
    > IsEntityPath for T
{}