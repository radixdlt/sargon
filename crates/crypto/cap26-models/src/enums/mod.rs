mod cap26_entity_kind;
mod cap26_key_kind;

pub use cap26_entity_kind::*;
pub use cap26_key_kind::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ascii_sums() {
        let ascii_sum = |s: &str| s.chars().fold(0, |acc, c| acc + c as u32);
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
    }
}
