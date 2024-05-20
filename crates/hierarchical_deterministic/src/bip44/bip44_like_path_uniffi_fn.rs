use crate::prelude::*;

#[uniffi::export]
pub fn new_bip44_like_path_from_index(index: HDPathValue) -> BIP44LikePath {
    BIP44LikePath::new(index)
}

#[uniffi::export]
pub fn new_bip44_like_path_from_string(
    string: String,
) -> Result<BIP44LikePath, CommonError> {
    BIP44LikePath::from_str(&string)
}

#[uniffi::export]
pub fn bip44_like_path_to_string(path: &BIP44LikePath) -> String {
    path.to_string()
}

#[uniffi::export]
pub fn new_bip44_like_path_sample() -> BIP44LikePath {
    BIP44LikePath::sample()
}

#[uniffi::export]
pub fn new_bip44_like_path_sample_other() -> BIP44LikePath {
    BIP44LikePath::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BIP44LikePath;

    #[test]
    fn test_new_bip44_like_path_from_index() {
        assert_eq!(new_bip44_like_path_from_index(0), SUT::new(0))
    }

    #[test]
    fn test_new_bip44_like_path_from_string() {
        let path = new_bip44_like_path_sample();
        assert_eq!(
            path,
            new_bip44_like_path_from_string(path.to_string()).unwrap()
        );

        let path_other = new_bip44_like_path_sample_other();
        assert_eq!(
            path_other,
            new_bip44_like_path_from_string(path_other.to_string()).unwrap()
        );
    }

    #[test]
    fn test_bip44_like_path_to_string() {
        let path = new_bip44_like_path_sample();
        assert_eq!(
            bip44_like_path_to_string(&path),
            new_bip44_like_path_from_string(path.to_string())
                .unwrap()
                .to_string()
        );

        let path_other = new_bip44_like_path_sample_other();
        assert_eq!(
            bip44_like_path_to_string(&path_other),
            new_bip44_like_path_from_string(path_other.to_string())
                .unwrap()
                .to_string()
        );
    }
}
