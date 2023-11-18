use nutype::nutype;

#[nutype(validate(max = 11))]
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppearanceID(u8);
