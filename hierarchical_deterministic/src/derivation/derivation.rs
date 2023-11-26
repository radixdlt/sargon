use slip10::*;

use crate::bip32::hd_path::HDPath;

use super::derivation_path_scheme::DerivationPathScheme;

pub trait Derivation: Sized {
    fn hd_path(&self) -> &HDPath;

    fn to_string(&self) -> String {
        self.hd_path().to_string()
    }
    fn scheme(&self) -> DerivationPathScheme;
}
