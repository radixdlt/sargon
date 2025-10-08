use crate::prelude::*;

/// The execution summary process not only determines the class of the manifest,
/// but also includes additional information about this class that the wallet
/// requires to display to the user.
#[derive(Clone, Debug, PartialEq, Eq, EnumAsInner)]
#[allow(clippy::large_enum_variant)]
pub enum DetailedManifestClass {
    /// A general manifest that involves any amount of arbitrary components
    /// and packages where nothing more concrete can be said about the manifest
    /// and its nature.
    ///
    /// No additional information is required beyond what the execution summary
    /// will provide.
    General,

    /// A manifest of a 1-to-1 transfer to a one-to-many transfer of resources.
    Transfer {
        /// When `true`, then this is a one-to-one transfer and the wallet can
        /// regard this as a "simple transfer" and communicate this information
        /// to the ledger hardware wallet. Otherwise, if `false`, then this is
        /// not a one-to-one transfer.
        is_one_to_one_transfer: bool,
    },

    /// A manifest where XRD is claimed from one or more validators.
    ValidatorClaim {
        /// The addresses of validators in the transaction
        validator_addresses: Vec<ValidatorAddress>,
        /// The claims observed in the transaction
        validator_claims: Vec<TrackedValidatorClaim>,
    },

    /// A manifest where XRD is staked to one or more validators.
    ValidatorStake {
        /// The addresses of validators in the transaction
        validator_addresses: Vec<ValidatorAddress>,
        /// The stake observed in the transaction
        validator_stakes: Vec<TrackedValidatorStake>,
    },

    /// A manifest where XRD is unstaked from one or more validators.
    ValidatorUnstake {
        /// The addresses of validators in the transaction
        validator_addresses: Vec<ValidatorAddress>,

        /// The data associated with the various claim NFTs
        claims_non_fungible_data: HashMap<NonFungibleGlobalId, UnstakeData>,
    },

    /// A manifest that updated the deposit settings of the account.
    AccountDepositSettingsUpdate {
        /// Updates to the resource preferences of the account deposit settings.
        /// account_address -> (resource_address -> Update<new_preference>)
        resource_preferences_updates: HashMap<
            AccountAddress,
            HashMap<ResourceAddress, ResourcePreferenceUpdate>,
        >,
        /// Changes to the account's deposit mode.
        /// account_address -> new_default_deposit_mode
        deposit_mode_updates: HashMap<AccountAddress, DepositRule>,
        /// Additions to the authorized depositors
        authorized_depositors_added:
            HashMap<AccountAddress, Vec<ResourceOrNonFungible>>,
        /// Removals from the authorized depositors
        authorized_depositors_removed:
            HashMap<AccountAddress, Vec<ResourceOrNonFungible>>,
    },

    /// A manifest that contributed some amount of resources to a liquidity
    /// pool that can be a one-resource pool, two-resource pool, or a
    /// multi-resource pool.
    PoolContribution {
        /// The addresses of the pools in the transaction
        pool_addresses: Vec<PoolAddress>,
        /// The contribution observed in the transaction
        pool_contributions: Vec<TrackedPoolContribution>,
    },

    /// A manifest that redeemed resources from a liquidity pool. Similar to
    /// contributions, this can be any of the three pool blueprints available
    /// in the pool package.
    PoolRedemption {
        /// The addresses of the pools in the transaction
        pool_addresses: Vec<PoolAddress>,

        /// The redemptions observed in the transaction
        pool_redemptions: Vec<TrackedPoolRedemption>,
    },

    /// A manifest that deletes accounts.
    DeleteAccounts {
        /// The addresses of the accounts that are being deleted
        account_addresses: Vec<AccountAddress>,
    },

    /// A manifest that is presented when a provisional security structure is applied
    /// to an entity
    SecurifyEntity {
        /// The entity address that is about to be securified
        entities: Vec<AddressOfAccountOrPersona>,
    },
    /// A manifest that starts the access controller recovery, it is either
    /// confirmed in the same transaction, or it is a timed recovery, currently
    /// the difference is not important.
    AccessControllerRecovery {
        ac_addresses: Vec<AccessControllerAddress>,
    },
    /// A manifest that stops the timed recovery.
    /// It can as well contain instructions to cancel the given recovery proposal.
    AccessControllerStopTimedRecovery {
        ac_addresses: Vec<AccessControllerAddress>,
    },
    AccessControllerConfirmTimedRecovery {
        ac_addresses: Vec<AccessControllerAddress>,
    },
}

impl DetailedManifestClass {
    pub fn kind(&self) -> DetailedManifestClassKind {
        match self {
            Self::General => DetailedManifestClassKind::General,
            Self::Transfer { .. } => DetailedManifestClassKind::Transfer,
            Self::ValidatorClaim { .. } => {
                DetailedManifestClassKind::ValidatorClaim
            }
            Self::ValidatorStake { .. } => {
                DetailedManifestClassKind::ValidatorStake
            }
            Self::ValidatorUnstake { .. } => {
                DetailedManifestClassKind::ValidatorUnstake
            }
            Self::AccountDepositSettingsUpdate { .. } => {
                DetailedManifestClassKind::AccountDepositSettingsUpdate
            }
            Self::PoolContribution { .. } => {
                DetailedManifestClassKind::PoolContribution
            }
            Self::PoolRedemption { .. } => {
                DetailedManifestClassKind::PoolRedemption
            }
            Self::DeleteAccounts { .. } => {
                DetailedManifestClassKind::DeleteAccounts
            }
            Self::SecurifyEntity { .. } => {
                DetailedManifestClassKind::SecurifyEntity
            }
            Self::AccessControllerRecovery { .. } => {
                DetailedManifestClassKind::AccessControllerRecovery
            }
            Self::AccessControllerConfirmTimedRecovery { .. } => {
                DetailedManifestClassKind::AccessControllerConfirmTimedRecovery
            }
            Self::AccessControllerStopTimedRecovery { .. } => {
                DetailedManifestClassKind::AccessControllerStopTimedRecovery
            }
        }
    }

    pub fn description(&self) -> String {
        self.kind().to_string()
    }
}

impl std::fmt::Display for DetailedManifestClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl DetailedManifestClass {
    /// Checks the manifest class is reserved for Wallet interactions only
    pub fn is_reserved(&self) -> bool {
        self.kind() == DetailedManifestClassKind::DeleteAccounts
            || self.kind() == DetailedManifestClassKind::SecurifyEntity
    }
}

impl DetailedManifestClass {
    pub fn new_from(
        ret_class: RetDetailedManifestClass,
        network_id: NetworkID,
    ) -> Option<Self> {
        match ret_class {
            RetDetailedManifestClass::General => Some(Self::General),

            RetDetailedManifestClass::Transfer {
                is_one_to_one_transfer,
            } => Some(Self::Transfer {
                is_one_to_one_transfer,
            }),

            RetDetailedManifestClass::PoolContribution(output) => {
                Some(Self::from((output, network_id)))
            }

            RetDetailedManifestClass::PoolRedemption(output) => {
                Some(Self::from((output, network_id)))
            }

            RetDetailedManifestClass::ValidatorStake(output) => {
                Some(Self::from((output, network_id)))
            }

            RetDetailedManifestClass::ValidatorUnstake(output) => {
                Some(Self::from((output, network_id)))
            }

            RetDetailedManifestClass::ValidatorClaimXrd(output) => {
                Some(Self::from((output, network_id)))
            }

            RetDetailedManifestClass::AccountDepositSettingsUpdate(output) => {
                Some(Self::from((output, network_id)))
            }

            RetDetailedManifestClass::GeneralSubintent => None,

            RetDetailedManifestClass::EntitySecurify(output) => {
                Some(Self::from((output, network_id)))
            }

            RetDetailedManifestClass::AccessControllerRecovery(output) => {
                Some(Self::from((output, network_id)))
            }

            RetDetailedManifestClass::AccessControllerConfirmTimedRecovery(
                output,
            ) => Some(Self::from((output, network_id))),

            RetDetailedManifestClass::AccessControllerStopTimedRecovery(
                output,
            ) => Some(Self::from((output, network_id))),
        }
    }
}

impl From<(RetPoolContributionOutput, NetworkID)> for DetailedManifestClass {
    fn from((output, n): (RetPoolContributionOutput, NetworkID)) -> Self {
        let pool_contributions: Vec<TrackedPoolContribution> =
            to_vec_network_aware(output.contribution_operations, n);
        let pool_addresses =
            pool_contributions.iter().map(|x| x.pool_address).collect();

        Self::PoolContribution {
            pool_addresses,
            pool_contributions,
        }
    }
}

impl From<(RetPoolRedemptionOutput, NetworkID)> for DetailedManifestClass {
    fn from((output, n): (RetPoolRedemptionOutput, NetworkID)) -> Self {
        let pool_redemptions: Vec<TrackedPoolRedemption> =
            to_vec_network_aware(output.redemption_operations, n);
        let pool_addresses =
            pool_redemptions.iter().map(|x| x.pool_address).collect();

        Self::PoolRedemption {
            pool_addresses,
            pool_redemptions,
        }
    }
}

impl From<(RetValidatorStakingOutput, NetworkID)> for DetailedManifestClass {
    fn from((output, n): (RetValidatorStakingOutput, NetworkID)) -> Self {
        let validator_stakes: Vec<TrackedValidatorStake> =
            to_vec_network_aware(output.stake_operations, n);
        let validator_addresses = validator_stakes
            .iter()
            .map(|x| x.validator_address)
            .collect();

        Self::ValidatorStake {
            validator_addresses,
            validator_stakes,
        }
    }
}

impl From<(RetValidatorUnstakingOutput, NetworkID)> for DetailedManifestClass {
    fn from((output, n): (RetValidatorUnstakingOutput, NetworkID)) -> Self {
        let validator_addresses: Vec<ScryptoComponentAddress> = output
            .unstake_operations
            .iter()
            .map(|x| x.validator_address)
            .collect();

        let claims_non_fungible_data = output
            .unstake_operations
            .iter()
            .flat_map(|op| {
                op.claim_nfts.iter().map(|(local_id, v)| {
                    let nft_resource_address = NonFungibleResourceAddress(
                        (op.claim_nft_address, n).into(),
                    );
                    (
                        NonFungibleGlobalId::new(
                            nft_resource_address,
                            local_id.clone().into(),
                        ),
                        UnstakeData::from(v.clone()),
                    )
                })
            })
            .collect::<_>();

        Self::ValidatorUnstake {
            validator_addresses: to_vec_network_aware(validator_addresses, n),
            claims_non_fungible_data,
        }
    }
}

impl From<(RetValidatorClaimingXrdOutput, NetworkID)>
    for DetailedManifestClass
{
    fn from((output, n): (RetValidatorClaimingXrdOutput, NetworkID)) -> Self {
        let validator_claims: Vec<TrackedValidatorClaim> =
            to_vec_network_aware(output.claim_operations, n);
        let validator_addresses = validator_claims
            .iter()
            .map(|x| x.validator_address)
            .collect();

        Self::ValidatorClaim {
            validator_addresses,
            validator_claims,
        }
    }
}

impl From<(RetAccountSettingsUpdateOutput, NetworkID)>
    for DetailedManifestClass
{
    fn from((output, n): (RetAccountSettingsUpdateOutput, NetworkID)) -> Self {
        let deposit_mode_updates: HashMap<AccountAddress, DepositRule> =
            filter_try_to_hashmap_network_aware_key(
                output.default_deposit_rule_updates,
                n,
            );

        type ResourcePreferenceUpdates = HashMap<
            AccountAddress,
            HashMap<ResourceAddress, ResourcePreferenceUpdate>,
        >;
        let resource_preferences_updates =
            output.resource_preference_updates.into_iter().fold(
                ResourcePreferenceUpdates::new(),
                |mut acc, ((account, resource), v)| {
                    let account_address =
                        AccountAddress::try_from((account, n));
                    let resource_address =
                        ResourceAddress::try_from((resource, n));
                    let update = ResourcePreferenceUpdate::from(v);
                    if let (Ok(account_address), Ok(resource_address)) =
                        (account_address, resource_address)
                    {
                        acc.entry(account_address)
                            .or_default()
                            .insert(resource_address, update);
                    };
                    acc
                },
            );

        let (authorized_depositors_added, authorized_depositors_removed): (
            Vec<_>,
            Vec<_>,
        ) = output
            .authorized_depositor_updates
            .into_iter()
            .filter_map(|((account, manifest_resource), op)| {
                let account_address =
                    AccountAddress::try_from((account, n)).ok()?;
                let resource =
                    ResourceOrNonFungible::try_from((manifest_resource, n))
                        .ok()?;
                Some(((account_address, resource), op))
            })
            .partition_map(|(account_resource_tuple, v)| match v {
                RetOperation::Added => Either::Left(account_resource_tuple),
                RetOperation::Removed => Either::Right(account_resource_tuple),
            });
        let authorized_depositors_added =
            authorized_depositors_added.into_iter().into_group_map();

        let authorized_depositors_removed =
            authorized_depositors_removed.into_iter().into_group_map();

        Self::AccountDepositSettingsUpdate {
            resource_preferences_updates,
            deposit_mode_updates,
            authorized_depositors_added,
            authorized_depositors_removed,
        }
    }
}

impl From<(RetEntitySecurifyOutput, NetworkID)> for DetailedManifestClass {
    fn from((output, n): (RetEntitySecurifyOutput, NetworkID)) -> Self {
        let accounts = filter_try_to_vec_network_aware::<_, AccountAddress>(
            output.securified_accounts,
            n,
        )
        .into_iter()
        .map(AddressOfAccountOrPersona::from)
        .collect::<Vec<_>>();

        let mut identities = filter_try_to_vec_network_aware::<
            _,
            IdentityAddress,
        >(output.securified_identities, n)
        .into_iter()
        .map(AddressOfAccountOrPersona::from)
        .collect::<Vec<_>>();

        let mut entities = accounts;
        entities.append(&mut identities);

        Self::SecurifyEntity { entities }
    }
}

impl From<(RetAccessControllerRecoveryOutput, NetworkID)>
    for DetailedManifestClass
{
    fn from(
        (output, n): (RetAccessControllerRecoveryOutput, NetworkID),
    ) -> Self {
        Self::AccessControllerRecovery {
            ac_addresses: to_vec_network_aware(output.access_controllers, n),
        }
    }
}

impl From<(RetAccessControllerConfirmTimedRecoveryOutput, NetworkID)>
    for DetailedManifestClass
{
    fn from(
        (output, n): (RetAccessControllerConfirmTimedRecoveryOutput, NetworkID),
    ) -> Self {
        Self::AccessControllerConfirmTimedRecovery {
            ac_addresses: to_vec_network_aware(output.access_controllers, n),
        }
    }
}

impl From<(RetAccessControllerStopTimedRecoveryOutput, NetworkID)>
    for DetailedManifestClass
{
    fn from(
        (output, n): (RetAccessControllerStopTimedRecoveryOutput, NetworkID),
    ) -> Self {
        Self::AccessControllerStopTimedRecovery {
            ac_addresses: to_vec_network_aware(output.access_controllers, n),
        }
    }
}

impl HasSampleValues for DetailedManifestClass {
    fn sample() -> Self {
        Self::General
    }

    fn sample_other() -> Self {
        Self::Transfer {
            is_one_to_one_transfer: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DetailedManifestClass;

    #[test]
    fn is_reserved_classification() {
        let general = SUT::General;
        let delete_accounts = SUT::DeleteAccounts {
            account_addresses: Vec::<_>::sample(),
        };

        pretty_assertions::assert_eq!(general.is_reserved(), false);
        pretty_assertions::assert_eq!(delete_accounts.is_reserved(), true);
    }

    #[test]
    fn kind() {
        let test = |s: SUT, exp: DetailedManifestClassKind| {
            assert_eq!(s.kind(), exp);
        };

        test(SUT::General, DetailedManifestClassKind::General);
        test(
            SUT::Transfer {
                is_one_to_one_transfer: false,
            },
            DetailedManifestClassKind::Transfer,
        );
        test(
            SUT::ValidatorClaim {
                validator_addresses: Vec::<_>::sample(),
                validator_claims: Vec::<_>::sample(),
            },
            DetailedManifestClassKind::ValidatorClaim,
        );
        test(
            SUT::ValidatorStake {
                validator_addresses: Vec::<_>::sample(),
                validator_stakes: Vec::<_>::sample(),
            },
            DetailedManifestClassKind::ValidatorStake,
        );
        test(
            SUT::ValidatorUnstake {
                validator_addresses: Vec::<_>::sample(),
                claims_non_fungible_data: HashMap::<_, _>::sample(),
            },
            DetailedManifestClassKind::ValidatorUnstake,
        );
        test(
            SUT::AccountDepositSettingsUpdate {
                resource_preferences_updates: HashMap::<_, _>::sample(),
                deposit_mode_updates: HashMap::<_, _>::sample(),
                authorized_depositors_added: HashMap::<_, _>::sample(),
                authorized_depositors_removed: HashMap::<_, _>::sample(),
            },
            DetailedManifestClassKind::AccountDepositSettingsUpdate,
        );
        test(
            SUT::PoolContribution {
                pool_addresses: Vec::<_>::sample(),
                pool_contributions: Vec::<_>::sample(),
            },
            DetailedManifestClassKind::PoolContribution,
        );
        test(
            SUT::PoolRedemption {
                pool_addresses: Vec::<_>::sample(),
                pool_redemptions: Vec::<_>::sample(),
            },
            DetailedManifestClassKind::PoolRedemption,
        );
        test(
            SUT::DeleteAccounts {
                account_addresses: Vec::<_>::sample(),
            },
            DetailedManifestClassKind::DeleteAccounts,
        );
        test(
            SUT::SecurifyEntity {
                entities: Vec::<_>::sample(),
            },
            DetailedManifestClassKind::SecurifyEntity,
        );
        test(
            SUT::AccessControllerRecovery {
                ac_addresses: Vec::<_>::sample(),
            },
            DetailedManifestClassKind::AccessControllerRecovery,
        );
        test(
            SUT::AccessControllerConfirmTimedRecovery {
                ac_addresses: Vec::<_>::sample(),
            },
            DetailedManifestClassKind::AccessControllerConfirmTimedRecovery,
        );
        test(
            SUT::AccessControllerStopTimedRecovery {
                ac_addresses: Vec::<_>::sample(),
            },
            DetailedManifestClassKind::AccessControllerStopTimedRecovery,
        );
    }
}
