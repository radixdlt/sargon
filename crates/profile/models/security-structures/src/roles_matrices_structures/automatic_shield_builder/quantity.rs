/// A tiny enum to make it possible to tell auto shield construction to
/// either assign ALL FactorSource matching some `FactorSelector` or only   
/// some fixed quantity (typically 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Quantity {
    All,
    One,
}

impl Quantity {
    pub(super) fn as_fixed(&self) -> Option<usize> {
        match self {
            Quantity::All => None,
            Quantity::One => Some(1),
        }
    }
}
