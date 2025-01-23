use crate::prelude::*;

#[derive(Clone, Default, derive_more::Debug, derive_more::Display)]
#[display("{}", self.to_cap43_string())]
#[debug("{}", self.to_cap43_string_debug())]
pub struct GetIDPath;

impl GetIDPath {
    pub const PATH: [HDPathComponent; 3] = [PURPOSE, COIN_TYPE, GET_ID_LAST];
}

impl From<GetIDPath> for HDPath {
    fn from(_: GetIDPath) -> Self {
        Self::new(Vec::from_iter(GetIDPath::PATH))
    }
}

impl GetIDPath {
    pub fn to_hd_path(&self) -> HDPath {
        HDPath::from(self.clone())
    }
}

impl ToCAP43String for GetIDPath {
    fn to_cap43_string(&self) -> String {
        self.to_hd_path().to_cap43_string()
    }
    fn to_cap43_string_debug(&self) -> String {
        self.to_hd_path().to_cap43_string_debug()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = GetIDPath;

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::default()), "m/44H/1022H/365H");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", SUT::default()), "m/44'/1022'/365'");
    }
}
