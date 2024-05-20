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
    uniffi::Enum,
)]
pub enum AddressFormat {
    Full,
    Raw,
    Default,
    Middle,
}
