use crate::prelude::*;

#[uniffi::export]
pub fn new_cap26_path_from_string(
    cap26_path_string: String,
) -> Result<CAP26Path, CommonError> {
    CAP26Path::from_str(&cap26_path_string)
}

#[uniffi::export]
pub fn default_get_id_path() -> GetIDPath {
    GetIDPath::default()
}

#[uniffi::export]
pub fn cap26_path_to_string(path: &CAP26Path) -> String {
    path.to_string()
}

#[uniffi::export]
pub fn new_account_path_sample() -> AccountPath {
    AccountPath::sample()
}

#[uniffi::export]
pub fn new_account_path_sample_other() -> AccountPath {
    AccountPath::sample_other()
}

#[uniffi::export]
pub fn new_identity_path_sample() -> IdentityPath {
    IdentityPath::sample()
}

#[uniffi::export]
pub fn new_identity_path_sample_other() -> IdentityPath {
    IdentityPath::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_path() {
        let path = CAP26Path::from(new_account_path_sample());
        assert_eq!(
            path,
            new_cap26_path_from_string(cap26_path_to_string(&path)).unwrap()
        );

        let path_other = CAP26Path::from(new_account_path_sample_other());
        assert_eq!(
            path_other,
            new_cap26_path_from_string(cap26_path_to_string(&path_other))
                .unwrap()
        );
    }

    #[test]
    fn test_identity_path() {
        let path = CAP26Path::from(new_identity_path_sample());
        assert_eq!(
            path,
            new_cap26_path_from_string(cap26_path_to_string(&path)).unwrap()
        );

        let path_other = CAP26Path::from(new_identity_path_sample_other());
        assert_eq!(
            path_other,
            new_cap26_path_from_string(cap26_path_to_string(&path_other))
                .unwrap()
        );
    }

    #[test]
    fn test_get_id_path() {
        let path = CAP26Path::from(default_get_id_path());
        assert_eq!(
            path,
            new_cap26_path_from_string(cap26_path_to_string(&path)).unwrap()
        );
    }
}
