use crate::prelude::*;
use sargon::DerivationPath as InternalDerivationPath;

/// A derivation path on either supported schemes, either Babylon (CAP26) or Olympia (BIP44Like).
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum DerivationPath {
    CAP26 { value: CAP26Path },
    BIP44Like { value: BIP44LikePath },
}

impl From<InternalDerivationPath> for DerivationPath {
    fn from(value: InternalDerivationPath) -> Self {
        match value {
            InternalDerivationPath::CAP26(value) => Self::CAP26 {
                value: value.into(),
            },
            InternalDerivationPath::BIP44Like(value) => Self::BIP44Like {
                value: value.into(),
            },
        }
    }
}

impl Into<InternalDerivationPath> for DerivationPath {
    fn into(self) -> InternalDerivationPath {
        match self {
            DerivationPath::CAP26 { value } => InternalDerivationPath::CAP26(value.into()),
            DerivationPath::BIP44Like { value } => {
                InternalDerivationPath::BIP44Like(value.into())
            }
        }
    }
}

#[uniffi::export]
pub fn new_derivation_path_sample() -> DerivationPath {
    InternalDerivationPath::sample().into()
}

#[uniffi::export]
pub fn new_derivation_path_sample_other() -> DerivationPath {
    InternalDerivationPath::sample_other().into()
}

#[uniffi::export]
pub fn new_derivation_path_from_string(
    string: String,
) -> Result<DerivationPath> {
    map_result_from_internal(InternalDerivationPath::from_str(&string))
}

#[uniffi::export]
pub fn derivation_path_to_hd_path(path: &DerivationPath) -> HDPath {
    path.into::<InternalDerivationPath>().hd_path().clone().into()
}

#[uniffi::export]
pub fn derivation_path_to_string(path: &DerivationPath) -> String {
    path.into::<InternalDerivationPath>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DerivationPath;

    #[test]
    fn test_from_str_cap26_account_path() {
        let s = "m/44H/1022H/1H/525H/1460H/0H";
        assert_eq!(
            new_derivation_path_from_string(s.to_owned()).unwrap(),
            SUT::from(AccountPath::sample())
        )
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_derivation_path_sample(),
                new_derivation_path_sample_other(),
                // duplicates should get removed
                new_derivation_path_sample(),
                new_derivation_path_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            SUT::sample().to_string(),
            derivation_path_to_string(&SUT::sample())
        )
    }

    #[test]
    fn test_to_hd_path() {
        assert_eq!(
            SUT::sample().hd_path(),
            &derivation_path_to_hd_path(&SUT::sample())
        )
    }
}
