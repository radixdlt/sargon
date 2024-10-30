use crate::prelude::*;

pub trait HasDerivationPathScheme {
    fn derivation_path_scheme() -> DerivationPathScheme;
    fn curve() -> SLIP10Curve {
        Self::derivation_path_scheme().curve()
    }
}
impl<T: NewEntityPath> HasDerivationPathScheme for T {
    fn derivation_path_scheme() -> DerivationPathScheme {
        DerivationPathScheme::Cap26
    }
}

pub trait HasDerivationPathSchemeObjectSafe {
    fn get_derivation_path_scheme(&self) -> DerivationPathScheme;
    fn curve(&self) -> SLIP10Curve {
        self.get_derivation_path_scheme().curve()
    }
}

impl<T: HasDerivationPathScheme> HasDerivationPathSchemeObjectSafe for T {
    fn get_derivation_path_scheme(&self) -> DerivationPathScheme {
        T::derivation_path_scheme()
    }
}
