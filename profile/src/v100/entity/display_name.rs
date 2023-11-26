use nutype::nutype;

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 20),
    derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Display,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash
    )
)]
pub struct DisplayName(String);

impl Default for DisplayName {
    fn default() -> Self {
        Self::new("Unnamed").expect("Default display name")
    }
}
