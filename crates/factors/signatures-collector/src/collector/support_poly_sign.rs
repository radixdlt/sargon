use crate::prelude::*;

/// Indicates entities that can enable poly sign.
/// `FactorSourceKind`s are the only entities that can be used on mono or poly sign.
/// Poly sign is designed to be used with factor kinds that can provide signatures with a single
/// authorisation.
///
/// One prime example of that is `FactorSourceKind::Device` which can provide
/// signatures for many device factor sources with a single biometrics request. Although disabled
/// for now.
pub(crate) trait SupportPolySign {
    fn support_poly_sign(&self) -> bool;
}

impl SupportPolySign for FactorSourceKind {
    fn support_poly_sign(&self) -> bool {
        false
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    pub fn all_kinds_supporting_mono_sign() {
        assert!(FactorSourceKind::all()
            .iter()
            .all(|kind| { !kind.support_poly_sign() }))
    }
}
