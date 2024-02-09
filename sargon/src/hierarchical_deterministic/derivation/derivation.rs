use crate::prelude::*;
pub trait Derivation: Sized {
    fn derivation_path(&self) -> DerivationPath;
    fn hd_path(&self) -> &HDPath;
    fn scheme(&self) -> DerivationPathScheme;

    fn bip32_string(&self) -> String {
        self.hd_path().to_string()
    }

    #[cfg(not(tarpaulin_include))] // false negative
    fn last_component(&self) -> &HDPathComponent {
        self.hd_path().components.last().unwrap()
    }
}
