use crate::prelude::*;
pub trait Derivation: Sized {
    fn scheme(&self) -> DerivationPathScheme;
    fn curve(&self) -> SLIP10Curve;
    fn derivation_path(&self) -> DerivationPath;
    fn hd_path(&self) -> &HDPath;

    fn bip32_string(&self) -> String {
        self.hd_path().to_string()
    }

    #[cfg(not(tarpaulin_include))] // false negative
    fn last_component(&self) -> &HDPathComponent {
        self.hd_path().components.last().unwrap()
    }
}
