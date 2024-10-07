/// A kind of factor list, either threshold, or override kind.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum FactorListKind {
    Threshold,
    Override,
}
