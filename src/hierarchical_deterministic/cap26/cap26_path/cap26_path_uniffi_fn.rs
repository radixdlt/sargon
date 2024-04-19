use crate::prelude::*;

#[uniffi::export]
pub fn new_cap26_path_from_string(
    string: String,
) -> Result<CAP26Path, CommonError> {
    CAP26Path::from_str(&string)
}

#[uniffi::export]
pub fn default_get_id_path() -> GetIDPath {
    GetIDPath::default()
}

#[uniffi::export]
pub fn cap26_path_to_string(path: &CAP26Path) -> String {
    path.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_path() {
        let path = CAP26Path::sample_account();
        assert_eq!(
            path,
            new_cap26_path_from_string(cap26_path_to_string(&path)).unwrap()
        );
    }

    #[test]
    fn test_identity_path() {
        let path = CAP26Path::sample_identity();
        assert_eq!(
            path,
            new_cap26_path_from_string(cap26_path_to_string(&path)).unwrap()
        );
    }

    #[test]
    fn test_get_id_path() {
        let path = CAP26Path::from(default_get_id_path());
        assert_eq!(
            path,
            new_cap26_path_from_string(String::from("m/44H/1022H/365H"))
                .unwrap()
        );
    }

    #[test]
    fn test_new_account_path() {
        let path = new_account_path(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            0,
        );
        assert_eq!(path.to_string(), "m/44H/1022H/1H/525H/1460H/0H")
    }

    #[test]
    fn test_new_identity_path() {
        let path = new_identity_path(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            0,
        );
        assert_eq!(path.to_string(), "m/44H/1022H/1H/618H/1460H/0H")
    }
}
