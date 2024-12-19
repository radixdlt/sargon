use crate::prelude::*;

/// Designates the purpose of initiating the `SignaturesCollector`. The collector can either
/// sign transactions, or prove ownership of entities such as personas or accounts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigningPurpose {
    /// Transactions can be signed using different roles from the MFA setup, for securified
    /// entities, or with `RoleKind::Primary` for unsecurified ones.
    SignTX { role_kind: RoleKind },

    /// Using `SignaturesCollector` for proving ownership of entities.
    ROLA,
}

impl SigningPurpose {
    pub fn sign_transaction(of_role_kind: RoleKind) -> Self {
        Self::SignTX {
            role_kind: of_role_kind,
        }
    }

    pub fn sign_transaction_primary() -> Self {
        Self::sign_transaction(RoleKind::Primary)
    }

    pub fn sign_transaction_recovery() -> Self {
        Self::sign_transaction(RoleKind::Recovery)
    }

    pub fn sign_transaction_confirmation() -> Self {
        Self::sign_transaction(RoleKind::Confirmation)
    }

    pub fn rola() -> Self {
        Self::ROLA
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SigningPurpose::SignTX;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SigningPurpose;

    #[test]
    fn test_for_primary_transaction() {
        assert_eq!(
            SUT::sign_transaction_primary(),
            SignTX {
                role_kind: RoleKind::Primary
            }
        )
    }

    #[test]
    fn test_for_recovery_transaction() {
        assert_eq!(
            SUT::sign_transaction_recovery(),
            SignTX {
                role_kind: RoleKind::Recovery
            }
        )
    }

    #[test]
    fn test_for_confirmation_transaction() {
        assert_eq!(
            SUT::sign_transaction_confirmation(),
            SignTX {
                role_kind: RoleKind::Confirmation
            }
        )
    }

    #[test]
    fn test_for_rola() {
        assert_eq!(SUT::rola(), SUT::ROLA)
    }
}
