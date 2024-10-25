mod cap26_entity_kind;
mod cap26_key_kind;
mod paths;

pub use cap26_entity_kind::*;
pub use cap26_key_kind::*;
pub use paths::*;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::GET_ID_CAP26_LOCAL;

    #[test]
    fn test_asciisum() {
        let ascii_sum =
            |s: &str| s.chars().fold(0, |acc, c| acc + c as u32);
        assert_eq!(
            ascii_sum("ACCOUNT"),
            CAP26EntityKind::Account.discriminant()
        );
        assert_eq!(
            ascii_sum("IDENTITY"),
            CAP26EntityKind::Identity.discriminant()
        );
        assert_eq!(
            ascii_sum("TRANSACTION_SIGNING"),
            CAP26KeyKind::TransactionSigning.discriminant()
        );
        assert_eq!(
            ascii_sum("AUTHENTICATION_SIGNING"),
            CAP26KeyKind::AuthenticationSigning.discriminant()
        );
        assert_eq!(ascii_sum("GETID"), GET_ID_CAP26_LOCAL as u32);
    }
}
