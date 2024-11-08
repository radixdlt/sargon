use crate::prelude::*;

pub trait HasKeyKind {
    fn key_kind() -> CAP26KeyKind;
}

pub trait HasKeyKindObjectSafe {
    fn get_key_kind(&self) -> CAP26KeyKind;
}

impl<T: HasKeyKind> HasKeyKindObjectSafe for T {
    fn get_key_kind(&self) -> CAP26KeyKind {
        T::key_kind()
    }
}

pub trait IsEntityPath:
    NewEntityPath
    + IsNetworkAware
    + HasEntityKind
    + HasKeyKindObjectSafe
    + Clone
    + Into<DerivationPath>
    + TryFrom<DerivationPath, Error = CommonError>
{
    // fn derivation_path(&self) -> DerivationPath {
    //     self.clone().into()
    // }
}
impl<
        T: NewEntityPath
            + Clone
            + IsNetworkAware
            + HasEntityKind
            + HasKeyKindObjectSafe
            + Into<DerivationPath>
            + TryFrom<DerivationPath, Error = CommonError>,
    > IsEntityPath for T
{
}

/// A specialized Hierarchical Deterministic FactorInstance used for transaction signing
/// and creation of virtual Accounts and Identities (Personas).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HDFactorInstanceTransactionSigning<T: IsEntityPath> {
    pub factor_source_id: FactorSourceIDFromHash,
    pub public_key: PublicKey,
    pub path: T,
}

impl<T: IsEntityPath> HDFactorInstanceTransactionSigning<T> {
    pub fn try_from_factor_instance(
        value: HierarchicalDeterministicFactorInstance,
    ) -> Result<Self> {
        T::try_from(value
            .derivation_path())
            .and_then(|p: T| {
                if p.get_entity_kind() != T::entity_kind() {
                    Err(CommonError::WrongEntityKindOfInFactorInstancesPath)
                } else {
                    Ok(p)
                }
            })
            .and_then(|p: T| {
                if !p.get_key_kind().is_transaction_signing() {
                    Err(CommonError::WrongKeyKindOfTransactionSigningFactorInstance)
                } else {
                    Ok(p)
                }
            })
            .map(|p| Self {
                factor_source_id: value.factor_source_id,
                public_key: value.public_key.public_key,
                path: p.clone(),
            })
    }
}

impl<T: IsEntityPath> HDFactorInstanceTransactionSigning<T> {
    pub fn network_id(&self) -> NetworkID {
        self.path.network_id()
    }

    pub fn public_key(&self) -> HierarchicalDeterministicPublicKey {
        HierarchicalDeterministicPublicKey::new(
            self.public_key,
            self.path.derivation_path(),
        )
    }
}

/// Just an alias for when `HDFactorInstanceTransactionSigning` is used to create a new Account.
pub type HDFactorInstanceAccountCreation =
    HDFactorInstanceTransactionSigning<AccountPath>;

/// Just an alias for when `HDFactorInstanceTransactionSigning` is used to create a new Account.
pub type HDFactorInstanceIdentityCreation =
    HDFactorInstanceTransactionSigning<IdentityPath>;

impl<T> HDFactorInstanceTransactionSigning<T>
where
    T: IsEntityPath,
{
    pub fn new(
        hd_factor_instance: HierarchicalDeterministicFactorInstance,
    ) -> Result<Self> {
        Self::try_from_factor_instance(hd_factor_instance)
    }
}

impl<E: IsEntityPath> From<HDFactorInstanceTransactionSigning<E>>
    for HierarchicalDeterministicFactorInstance
{
    fn from(value: HDFactorInstanceTransactionSigning<E>) -> Self {
        HierarchicalDeterministicFactorInstance::new(
            value.clone().factor_source_id,
            value.public_key(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn account_creation_valid() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::sample_ed25519(),
            AccountPath::sample().into(),
        );
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            hd_key,
        );
        assert_eq!(
            HDFactorInstanceAccountCreation::new(hd_fi)
                .unwrap()
                .path
                .get_key_kind(),
            CAP26KeyKind::TransactionSigning
        );
    }

    #[test]
    fn account_creation_wrong_entity_kind() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::sample_ed25519(),
            IdentityPath::sample().into(),
        );
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            hd_key,
        );
        assert!(HDFactorInstanceAccountCreation::new(hd_fi).is_err());
    }

    #[test]
    fn account_creation_wrong_key_kind() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::sample_ed25519(),
            AccountPath::new(
                NetworkID::Mainnet,
                CAP26KeyKind::AuthenticationSigning,
                UnsecurifiedHardened::from_local_key_space(0).unwrap(),
            )
            .into(),
        );
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            hd_key,
        );
        assert_eq!(
            HDFactorInstanceAccountCreation::new(hd_fi),
            Err(CommonError::WrongKeyKindOfTransactionSigningFactorInstance)
        );
    }

    #[test]
    fn identity_creation_valid() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::sample_ed25519(),
            IdentityPath::sample().into(),
        );
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            hd_key,
        );
        assert_eq!(
            HDFactorInstanceIdentityCreation::new(hd_fi)
                .unwrap()
                .path
                .get_key_kind(),
            CAP26KeyKind::TransactionSigning
        );
    }

    #[test]
    fn identity_creation_wrong_entity_kind() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::sample_ed25519(),
            AccountPath::sample().into(),
        );
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            hd_key,
        );
        assert!(HDFactorInstanceIdentityCreation::new(hd_fi).is_err());
    }

    #[test]
    fn identity_creation_wrong_key_kind() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::sample_ed25519(),
            IdentityPath::new(
                NetworkID::Mainnet,
                CAP26KeyKind::AuthenticationSigning,
                Hardened::from_local_key_space(0, IsSecurified(false)).unwrap(),
            )
            .into(),
        );
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            hd_key,
        );
        assert_eq!(
            HDFactorInstanceIdentityCreation::new(hd_fi),
            Err(CommonError::WrongKeyKindOfTransactionSigningFactorInstance)
        );
    }
}
