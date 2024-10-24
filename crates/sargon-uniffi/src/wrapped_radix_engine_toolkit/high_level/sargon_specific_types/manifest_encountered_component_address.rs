use crate::prelude::*;

address_union!(
    /// A tagged union of all the encountered addresses in the manifest.
    /// This is to be primarily used for the "using dApps" section of the wallet's tx review screen.
    enum ManifestEncounteredComponentAddress: component, locker
);
