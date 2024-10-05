#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    enum_iterator::Sequence,
)]
pub enum AddressFormat {
    Full,
    Raw,
    Default,
}
