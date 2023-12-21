use crate::bip32::{HDPath, HDPathComponent};

use super::{derivation_path::DerivationPath, derivation_path_scheme::DerivationPathScheme};

pub trait Derivation: Sized {
    fn derivation_path(&self) -> DerivationPath;
    fn hd_path(&self) -> &HDPath;

    fn to_string(&self) -> String {
        self.hd_path().to_string()
    }

    fn scheme(&self) -> DerivationPathScheme;

    #[cfg(not(tarpaulin_include))] // false negative
    fn last_component(&self) -> &HDPathComponent {
        self.hd_path().components().last().unwrap()
    }
}
