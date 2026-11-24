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
    /// A manifest that proposes new rules for access controllers and confirms
    /// them in the same transaction: the rules change once it's committed.
    AccessControllerRecovery {
        ac_addresses: Vec<AccessControllerAddress>,
        /// The recoveries proposed and confirmed in the manifest, with the
        /// rules they enact.
        recovery_proposals: Vec<AccessControllerRecoveryProposal>,
    },
    /// A manifest that starts a timed recovery of access controllers: the
    /// recovery role proposes rules without confirming them.
    AccessControllerTimedRecovery {
        ac_addresses: Vec<AccessControllerAddress>,
        timed_recoveries: Vec<AccessControllerTimedRecoveryProposal>,
    },
    /// A manifest that confirms recoveries already proposed on access
    /// controllers.
    AccessControllerConfirmRecovery {
        ac_addresses: Vec<AccessControllerAddress>,
        confirmations: Vec<AccessControllerRecoveryConfirmation>,
    },
    /// A manifest that stops or cancels recoveries already proposed on access
    /// controllers.
    AccessControllerCancelRecovery {
        ac_addresses: Vec<AccessControllerAddress>,
        cancellations: Vec<AccessControllerRecoveryCancellation>,
    },
    /// A manifest that locks or unlocks the primary role of access
    /// controllers.
    AccessControllerPrimaryRoleLock {
        ac_addresses: Vec<AccessControllerAddress>,
        operations: Vec<AccessControllerPrimaryRoleLockOperation>,
    },
    /// A manifest that tops up the recovery fee vaults of access controllers
    /// with XRD withdrawn from accounts, or withdraws XRD from them into
    /// accounts. Only one side is filled: `contributions` and
    /// `account_withdrawals` for a top-up, `withdrawals` and
    /// `account_deposits` for a withdrawal.
    AccessControllerRecoveryFee {
        ac_addresses: Vec<AccessControllerAddress>,
        contributions: Vec<AccessControllerXrdTransfer>,
        withdrawals: Vec<AccessControllerXrdTransfer>,
        account_withdrawals: Vec<AccountXrdTransfer>,
        account_deposits: Vec<AccountXrdTransfer>,
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
            Self::AccessControllerTimedRecovery { .. } => {
                DetailedManifestClassKind::AccessControllerTimedRecovery
            }
            Self::AccessControllerConfirmRecovery { .. } => {
                DetailedManifestClassKind::AccessControllerConfirmRecovery
            }
            Self::AccessControllerCancelRecovery { .. } => {
                DetailedManifestClassKind::AccessControllerCancelRecovery
            }
            Self::AccessControllerPrimaryRoleLock { .. } => {
                DetailedManifestClassKind::AccessControllerPrimaryRoleLock
            }
            Self::AccessControllerRecoveryFee { .. } => {
                DetailedManifestClassKind::AccessControllerRecoveryFee
            }
        }
    }

    /// The access controllers an access controller class operates on, or
    /// `None` for other classes.
    pub fn access_controllers(&self) -> Option<&[AccessControllerAddress]> {
        match self {
            Self::AccessControllerRecovery { ac_addresses, .. }
            | Self::AccessControllerTimedRecovery { ac_addresses, .. }
            | Self::AccessControllerConfirmRecovery { ac_addresses, .. }
            | Self::AccessControllerCancelRecovery { ac_addresses, .. }
            | Self::AccessControllerPrimaryRoleLock { ac_addresses, .. }
            | Self::AccessControllerRecoveryFee { ac_addresses, .. } => {
                Some(ac_addresses)
            }
            _ => None,
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

            RetDetailedManifestClass::AccessControllerTimedRecovery(output) => {
                Some(Self::from((output, network_id)))
            }

            RetDetailedManifestClass::AccessControllerConfirmRecovery(
                output,
            ) => Some(Self::from((output, network_id))),

            RetDetailedManifestClass::AccessControllerCancelRecovery(
                output,
            ) => Some(Self::from((output, network_id))),

            RetDetailedManifestClass::AccessControllerPrimaryRoleLock(
                output,
            ) => Some(Self::from((output, network_id))),

            RetDetailedManifestClass::AccessControllerRecoveryFee(output) => {
                Some(Self::from((output, network_id)))
            }
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
        let (accounts, identities): (Vec<_>, Vec<_>) = output
            .securify_operations
            .into_iter()
            .filter_map(|operation| {
                let node_id = *operation.entity_address.as_node_id();
                AccountAddress::new_from_node_id(node_id, n)
                    .map(AddressOfAccountOrPersona::from)
                    .map(Either::Left)
                    .or_else(|_| {
                        IdentityAddress::new_from_node_id(node_id, n)
                            .map(AddressOfAccountOrPersona::from)
                            .map(Either::Right)
                    })
                    .ok()
            })
            .partition_map(|entity| entity);

        // Accounts first, then identities, as before the toolkit merged them
        // into a single list of securify operations.
        let entities = accounts.into_iter().chain(identities).collect();

        Self::SecurifyEntity { entities }
    }
}

impl DetailedManifestClass {
    /// Marks the user's keys in the rules an access controller class shows,
    /// which the classification shows as unknown, see
    /// [`AccessControllerRuleSetForDisplay::with_own_factor_sources`].
    pub fn with_own_factor_sources(
        self,
        own_factor_source: &impl Fn(&NonFungibleGlobalId) -> Option<FactorSource>,
    ) -> Self {
        let mark = |rules: AccessControllerRuleSetForDisplay| {
            rules.with_own_factor_sources(own_factor_source)
        };
        match self {
            Self::AccessControllerRecovery {
                ac_addresses,
                recovery_proposals,
            } => Self::AccessControllerRecovery {
                ac_addresses,
                recovery_proposals: recovery_proposals
                    .into_iter()
                    .map(|proposal| AccessControllerRecoveryProposal {
                        proposed_rules: mark(proposal.proposed_rules),
                        ..proposal
                    })
                    .collect(),
            },
            Self::AccessControllerTimedRecovery {
                ac_addresses,
                timed_recoveries,
            } => Self::AccessControllerTimedRecovery {
                ac_addresses,
                timed_recoveries: timed_recoveries
                    .into_iter()
                    .map(|recovery| AccessControllerTimedRecoveryProposal {
                        proposed_rules: mark(recovery.proposed_rules),
                        ..recovery
                    })
                    .collect(),
            },
            Self::AccessControllerConfirmRecovery {
                ac_addresses,
                confirmations,
            } => Self::AccessControllerConfirmRecovery {
                ac_addresses,
                confirmations: confirmations
                    .into_iter()
                    .map(|confirmation| AccessControllerRecoveryConfirmation {
                        proposed_rules: mark(confirmation.proposed_rules),
                        ..confirmation
                    })
                    .collect(),
            },
            Self::AccessControllerCancelRecovery {
                ac_addresses,
                cancellations,
            } => Self::AccessControllerCancelRecovery {
                ac_addresses,
                cancellations: cancellations
                    .into_iter()
                    .map(|cancellation| AccessControllerRecoveryCancellation {
                        stopped_timed_recovery: cancellation
                            .stopped_timed_recovery
                            .map(mark),
                        ..cancellation
                    })
                    .collect(),
            },
            class => class,
        }
    }
}

/// What the OS reads from the ledger about an access controller an access
/// controller class operates on.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AccessControllerOnLedger {
    /// The account or persona the access controller controls.
    pub controlled_entity: Option<AddressOfAccountOrPersona>,
    /// The access controller's timed recovery delay.
    pub timed_recovery_delay_in_minutes: Option<u32>,
}

impl DetailedManifestClass {
    /// Sets what the classification doesn't know about the access controllers
    /// an access controller class operates on, from `on_ledger`: the entity
    /// each controls and, where rules are proposed or confirmed, its current
    /// delay, which it keeps whatever delay the manifest names.
    pub fn with_access_controllers_on_ledger(
        self,
        on_ledger: &impl Fn(&AccessControllerAddress) -> AccessControllerOnLedger,
    ) -> Self {
        let entity = |access_controller: &AccessControllerAddress| {
            on_ledger(access_controller).controlled_entity
        };
        match self {
            Self::AccessControllerRecovery {
                ac_addresses,
                recovery_proposals,
            } => Self::AccessControllerRecovery {
                ac_addresses,
                recovery_proposals: recovery_proposals
                    .into_iter()
                    .map(|proposal| {
                        let AccessControllerOnLedger {
                            controlled_entity,
                            timed_recovery_delay_in_minutes,
                        } = on_ledger(&proposal.access_controller);
                        AccessControllerRecoveryProposal {
                            controlled_entity,
                            current_timed_recovery_delay_in_minutes:
                                timed_recovery_delay_in_minutes,
                            ..proposal
                        }
                    })
                    .collect(),
            },
            Self::AccessControllerTimedRecovery {
                ac_addresses,
                timed_recoveries,
            } => Self::AccessControllerTimedRecovery {
                ac_addresses,
                timed_recoveries: timed_recoveries
                    .into_iter()
                    .map(|recovery| {
                        let AccessControllerOnLedger {
                            controlled_entity,
                            timed_recovery_delay_in_minutes,
                        } = on_ledger(&recovery.access_controller);
                        AccessControllerTimedRecoveryProposal {
                            controlled_entity,
                            current_timed_recovery_delay_in_minutes:
                                timed_recovery_delay_in_minutes,
                            ..recovery
                        }
                    })
                    .collect(),
            },
            Self::AccessControllerConfirmRecovery {
                ac_addresses,
                confirmations,
            } => Self::AccessControllerConfirmRecovery {
                ac_addresses,
                confirmations: confirmations
                    .into_iter()
                    .map(|confirmation| {
                        let AccessControllerOnLedger {
                            controlled_entity,
                            timed_recovery_delay_in_minutes,
                        } = on_ledger(&confirmation.access_controller);
                        AccessControllerRecoveryConfirmation {
                            controlled_entity,
                            current_timed_recovery_delay_in_minutes:
                                timed_recovery_delay_in_minutes,
                            ..confirmation
                        }
                    })
                    .collect(),
            },
            Self::AccessControllerCancelRecovery {
                ac_addresses,
                cancellations,
            } => Self::AccessControllerCancelRecovery {
                ac_addresses,
                cancellations: cancellations
                    .into_iter()
                    .map(|cancellation| AccessControllerRecoveryCancellation {
                        controlled_entity: entity(
                            &cancellation.access_controller,
                        ),
                        ..cancellation
                    })
                    .collect(),
            },
            Self::AccessControllerPrimaryRoleLock {
                ac_addresses,
                operations,
            } => Self::AccessControllerPrimaryRoleLock {
                ac_addresses,
                operations: operations
                    .into_iter()
                    .map(|operation| AccessControllerPrimaryRoleLockOperation {
                        controlled_entity: entity(&operation.access_controller),
                        ..operation
                    })
                    .collect(),
            },
            Self::AccessControllerRecoveryFee {
                ac_addresses,
                contributions,
                withdrawals,
                account_withdrawals,
                account_deposits,
            } => {
                let with_entities =
                    |transfers: Vec<AccessControllerXrdTransfer>| {
                        transfers
                            .into_iter()
                            .map(|transfer| AccessControllerXrdTransfer {
                                controlled_entity: entity(
                                    &transfer.access_controller,
                                ),
                                ..transfer
                            })
                            .collect()
                    };
                Self::AccessControllerRecoveryFee {
                    ac_addresses,
                    contributions: with_entities(contributions),
                    withdrawals: with_entities(withdrawals),
                    account_withdrawals,
                    account_deposits,
                }
            }
            class => class,
        }
    }
}

/// The distinct access controllers of `items`, in order.
fn distinct_access_controllers<T>(
    items: &[T],
    access_controller: impl Fn(&T) -> AccessControllerAddress,
) -> Vec<AccessControllerAddress> {
    items.iter().map(access_controller).unique().collect()
}

impl From<(RetAccessControllerRecoveryOutput, NetworkID)>
    for DetailedManifestClass
{
    fn from(
        (output, n): (RetAccessControllerRecoveryOutput, NetworkID),
    ) -> Self {
        let recovery_proposals = output
            .recovery_proposals
            .into_iter()
            .map(|proposal| {
                AccessControllerRecoveryProposal::from((proposal, n))
            })
            .collect_vec();
        let ac_addresses =
            distinct_access_controllers(&recovery_proposals, |proposal| {
                proposal.access_controller
            });

        Self::AccessControllerRecovery {
            ac_addresses,
            recovery_proposals,
        }
    }
}

impl From<(RetAccessControllerTimedRecoveryOutput, NetworkID)>
    for DetailedManifestClass
{
    fn from(
        (output, n): (RetAccessControllerTimedRecoveryOutput, NetworkID),
    ) -> Self {
        let timed_recoveries = output
            .timed_recoveries
            .into_iter()
            .map(|recovery| {
                AccessControllerTimedRecoveryProposal::from((recovery, n))
            })
            .collect_vec();
        Self::AccessControllerTimedRecovery {
            ac_addresses: distinct_access_controllers(
                &timed_recoveries,
                |recovery| recovery.access_controller,
            ),
            timed_recoveries,
        }
    }
}

impl From<(RetAccessControllerConfirmRecoveryOutput, NetworkID)>
    for DetailedManifestClass
{
    fn from(
        (output, n): (RetAccessControllerConfirmRecoveryOutput, NetworkID),
    ) -> Self {
        let confirmations = output
            .confirmations
            .into_iter()
            .map(|confirmation| {
                AccessControllerRecoveryConfirmation::from((confirmation, n))
            })
            .collect_vec();
        Self::AccessControllerConfirmRecovery {
            ac_addresses: distinct_access_controllers(
                &confirmations,
                |confirmation| confirmation.access_controller,
            ),
            confirmations,
        }
    }
}

impl From<(RetAccessControllerCancelRecoveryOutput, NetworkID)>
    for DetailedManifestClass
{
    fn from(
        (output, n): (RetAccessControllerCancelRecoveryOutput, NetworkID),
    ) -> Self {
        let cancellations = output
            .cancellations
            .into_iter()
            .map(|cancellation| {
                AccessControllerRecoveryCancellation::from((cancellation, n))
            })
            .collect_vec();
        Self::AccessControllerCancelRecovery {
            ac_addresses: distinct_access_controllers(
                &cancellations,
                |cancellation| cancellation.access_controller,
            ),
            cancellations,
        }
    }
}

impl From<(RetAccessControllerPrimaryRoleLockOutput, NetworkID)>
    for DetailedManifestClass
{
    fn from(
        (output, n): (RetAccessControllerPrimaryRoleLockOutput, NetworkID),
    ) -> Self {
        let operations = output
            .operations
            .into_iter()
            .map(|operation| {
                AccessControllerPrimaryRoleLockOperation::from((operation, n))
            })
            .collect_vec();
        Self::AccessControllerPrimaryRoleLock {
            ac_addresses: distinct_access_controllers(
                &operations,
                |operation| operation.access_controller,
            ),
            operations,
        }
    }
}

impl From<(RetAccessControllerRecoveryFeeOutput, NetworkID)>
    for DetailedManifestClass
{
    fn from(
        (output, n): (RetAccessControllerRecoveryFeeOutput, NetworkID),
    ) -> Self {
        let access_controller_transfers = |transfers: Vec<_>| {
            transfers
                .into_iter()
                .map(|transfer| {
                    AccessControllerXrdTransfer::from((transfer, n))
                })
                .collect_vec()
        };
        let account_transfers = |transfers: Vec<_>| {
            transfers
                .into_iter()
                .map(|transfer| AccountXrdTransfer::from((transfer, n)))
                .collect_vec()
        };
        let contributions = access_controller_transfers(output.contributions);
        let withdrawals = access_controller_transfers(output.withdrawals);
        let ac_addresses = distinct_access_controllers(
            &[contributions.as_slice(), withdrawals.as_slice()].concat(),
            |transfer| transfer.access_controller,
        );
        Self::AccessControllerRecoveryFee {
            ac_addresses,
            contributions,
            withdrawals,
            account_withdrawals: account_transfers(output.account_withdrawals),
            account_deposits: account_transfers(output.account_deposits),
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
                recovery_proposals: vec![
                    AccessControllerRecoveryProposal::sample(),
                ],
            },
            DetailedManifestClassKind::AccessControllerRecovery,
        );
        test(
            SUT::AccessControllerTimedRecovery {
                ac_addresses: Vec::<_>::sample(),
                timed_recoveries: vec![
                    AccessControllerTimedRecoveryProposal::sample(),
                ],
            },
            DetailedManifestClassKind::AccessControllerTimedRecovery,
        );
        test(
            SUT::AccessControllerConfirmRecovery {
                ac_addresses: Vec::<_>::sample(),
                confirmations: vec![
                    AccessControllerRecoveryConfirmation::sample(),
                ],
            },
            DetailedManifestClassKind::AccessControllerConfirmRecovery,
        );
        test(
            SUT::AccessControllerCancelRecovery {
                ac_addresses: Vec::<_>::sample(),
                cancellations: vec![
                    AccessControllerRecoveryCancellation::sample(),
                ],
            },
            DetailedManifestClassKind::AccessControllerCancelRecovery,
        );
        test(
            SUT::AccessControllerPrimaryRoleLock {
                ac_addresses: Vec::<_>::sample(),
                operations: vec![
                    AccessControllerPrimaryRoleLockOperation::sample(),
                ],
            },
            DetailedManifestClassKind::AccessControllerPrimaryRoleLock,
        );
        test(
            SUT::AccessControllerRecoveryFee {
                ac_addresses: Vec::<_>::sample(),
                contributions: vec![AccessControllerXrdTransfer::sample()],
                withdrawals: vec![],
                account_withdrawals: vec![AccountXrdTransfer::sample()],
                account_deposits: vec![],
            },
            DetailedManifestClassKind::AccessControllerRecoveryFee,
        );
    }

    #[test]
    fn recovery_carries_each_proposal_and_deduplicates_access_controllers() {
        use radix_engine_interface::blueprints::access_controller::{
            Proposer as ScryptoProposer, RuleSet as ScryptoRuleSet,
        };
        use radix_engine_interface::prelude::{
            AccessRule as ScryptoAccessRule,
            ComponentAddress as ScryptoComponentAddress,
        };

        let access_controller = AccessControllerAddress::sample();
        let scrypto_access_controller = ScryptoComponentAddress::new_or_panic(
            access_controller.scrypto().as_node_id().0,
        );
        let rule_set = ScryptoRuleSet {
            primary_role: ScryptoAccessRule::AllowAll,
            recovery_role: ScryptoAccessRule::DenyAll,
            confirmation_role: ScryptoAccessRule::AllowAll,
        };
        let proposal = |proposer, delay| RetAccessControllerRecoveryProposal {
            access_controller: scrypto_access_controller,
            proposer,
            rule_set: rule_set.clone(),
            timed_recovery_delay_in_minutes: delay,
        };
        let output = RetAccessControllerRecoveryOutput {
            recovery_proposals: vec![
                proposal(ScryptoProposer::Primary, None),
                proposal(ScryptoProposer::Recovery, Some(10)),
            ],
        };

        let proposed_rules = AccessControllerRuleSetForDisplay {
            primary_role: AccessRequirementForDisplay::Anyone,
            recovery_role: AccessRequirementForDisplay::NoOne,
            confirmation_role: AccessRequirementForDisplay::Anyone,
        };
        assert_eq!(
            SUT::from((output, NetworkID::Mainnet)),
            SUT::AccessControllerRecovery {
                ac_addresses: vec![access_controller],
                recovery_proposals: vec![
                    AccessControllerRecoveryProposal {
                        access_controller,
                        proposer: RecoveryProposer::Primary,
                        proposed_rules: proposed_rules.clone(),
                        timed_recovery_delay_in_minutes: None,
                        current_timed_recovery_delay_in_minutes: None,
                        controlled_entity: None,
                    },
                    AccessControllerRecoveryProposal {
                        access_controller,
                        proposer: RecoveryProposer::Recovery,
                        proposed_rules,
                        timed_recovery_delay_in_minutes: Some(10),
                        current_timed_recovery_delay_in_minutes: None,
                        controlled_entity: None,
                    },
                ],
            }
        );
    }

    #[test]
    fn timed_recovery_shows_the_current_delay_and_the_entity_on_ledger() {
        let timed_recovery = AccessControllerTimedRecoveryProposal {
            current_timed_recovery_delay_in_minutes: None,
            controlled_entity: None,
            ..AccessControllerTimedRecoveryProposal::sample()
        };
        let class = SUT::AccessControllerTimedRecovery {
            ac_addresses: vec![timed_recovery.access_controller],
            timed_recoveries: vec![timed_recovery.clone()],
        };
        let entity = AddressOfAccountOrPersona::from(AccountAddress::sample());

        let class = class.with_access_controllers_on_ledger(&|_| {
            AccessControllerOnLedger {
                controlled_entity: Some(entity),
                timed_recovery_delay_in_minutes: Some(42),
            }
        });

        assert_eq!(
            class.access_controllers(),
            Some([timed_recovery.access_controller].as_slice())
        );
        assert_eq!(
            class,
            SUT::AccessControllerTimedRecovery {
                ac_addresses: vec![timed_recovery.access_controller],
                timed_recoveries: vec![AccessControllerTimedRecoveryProposal {
                    current_timed_recovery_delay_in_minutes: Some(42),
                    controlled_entity: Some(entity),
                    ..timed_recovery
                }],
            }
        );
    }

    /// The delay a recovery or a confirmation names isn't applied, so both
    /// carry the one the access controller keeps.
    #[test]
    fn recovery_and_confirmation_show_the_current_delay_on_ledger() {
        let on_ledger =
            |_: &AccessControllerAddress| AccessControllerOnLedger {
                controlled_entity: None,
                timed_recovery_delay_in_minutes: Some(1),
            };
        let proposal = AccessControllerRecoveryProposal {
            timed_recovery_delay_in_minutes: Some(20160),
            current_timed_recovery_delay_in_minutes: None,
            controlled_entity: None,
            ..AccessControllerRecoveryProposal::sample()
        };
        let confirmation = AccessControllerRecoveryConfirmation {
            timed_recovery_delay_in_minutes: Some(20160),
            current_timed_recovery_delay_in_minutes: None,
            controlled_entity: None,
            ..AccessControllerRecoveryConfirmation::sample()
        };

        assert_eq!(
            SUT::AccessControllerRecovery {
                ac_addresses: Vec::new(),
                recovery_proposals: vec![proposal.clone()],
            }
            .with_access_controllers_on_ledger(&on_ledger),
            SUT::AccessControllerRecovery {
                ac_addresses: Vec::new(),
                recovery_proposals: vec![AccessControllerRecoveryProposal {
                    current_timed_recovery_delay_in_minutes: Some(1),
                    ..proposal
                }],
            }
        );
        assert_eq!(
            SUT::AccessControllerConfirmRecovery {
                ac_addresses: Vec::new(),
                confirmations: vec![confirmation.clone()],
            }
            .with_access_controllers_on_ledger(&on_ledger),
            SUT::AccessControllerConfirmRecovery {
                ac_addresses: Vec::new(),
                confirmations: vec![AccessControllerRecoveryConfirmation {
                    current_timed_recovery_delay_in_minutes: Some(1),
                    ..confirmation
                }],
            }
        );
    }

    #[test]
    fn every_access_controller_class_shows_the_entity_on_ledger() {
        let entity = AddressOfAccountOrPersona::from(AccountAddress::sample());
        let on_ledger =
            |_: &AccessControllerAddress| AccessControllerOnLedger {
                controlled_entity: Some(entity),
                timed_recovery_delay_in_minutes: None,
            };
        let classes = [
            SUT::AccessControllerRecovery {
                ac_addresses: Vec::new(),
                recovery_proposals: vec![AccessControllerRecoveryProposal {
                    controlled_entity: None,
                    ..AccessControllerRecoveryProposal::sample()
                }],
            },
            SUT::AccessControllerConfirmRecovery {
                ac_addresses: Vec::new(),
                confirmations: vec![AccessControllerRecoveryConfirmation {
                    controlled_entity: None,
                    ..AccessControllerRecoveryConfirmation::sample()
                }],
            },
            SUT::AccessControllerCancelRecovery {
                ac_addresses: Vec::new(),
                cancellations: vec![AccessControllerRecoveryCancellation {
                    controlled_entity: None,
                    ..AccessControllerRecoveryCancellation::sample()
                }],
            },
            SUT::AccessControllerPrimaryRoleLock {
                ac_addresses: Vec::new(),
                operations: vec![AccessControllerPrimaryRoleLockOperation {
                    controlled_entity: None,
                    ..AccessControllerPrimaryRoleLockOperation::sample()
                }],
            },
            SUT::AccessControllerRecoveryFee {
                ac_addresses: Vec::new(),
                contributions: vec![AccessControllerXrdTransfer {
                    controlled_entity: None,
                    ..AccessControllerXrdTransfer::sample()
                }],
                withdrawals: Vec::new(),
                account_withdrawals: vec![AccountXrdTransfer::sample()],
                account_deposits: Vec::new(),
            },
        ];
        for class in classes {
            let entities = match class
                .with_access_controllers_on_ledger(&on_ledger)
            {
                SUT::AccessControllerRecovery {
                    recovery_proposals, ..
                } => recovery_proposals
                    .into_iter()
                    .map(|x| x.controlled_entity)
                    .collect_vec(),
                SUT::AccessControllerConfirmRecovery {
                    confirmations, ..
                } => confirmations
                    .into_iter()
                    .map(|x| x.controlled_entity)
                    .collect_vec(),
                SUT::AccessControllerCancelRecovery {
                    cancellations, ..
                } => cancellations
                    .into_iter()
                    .map(|x| x.controlled_entity)
                    .collect_vec(),
                SUT::AccessControllerPrimaryRoleLock { operations, .. } => {
                    operations
                        .into_iter()
                        .map(|x| x.controlled_entity)
                        .collect_vec()
                }
                SUT::AccessControllerRecoveryFee { contributions, .. } => {
                    contributions
                        .into_iter()
                        .map(|x| x.controlled_entity)
                        .collect_vec()
                }
                class => panic!("Unexpected {class:?}"),
            };
            assert_eq!(entities, vec![Some(entity)]);
        }
    }

    #[test]
    fn recovery_fee_lists_each_access_controller_once() {
        use radix_engine_interface::prelude::ComponentAddress as ScryptoComponentAddress;

        let access_controller = AccessControllerAddress::sample();
        let scrypto_access_controller = ScryptoComponentAddress::new_or_panic(
            access_controller.scrypto().as_node_id().0,
        );
        let account = AccountAddress::sample();
        let scrypto_account = ScryptoComponentAddress::new_or_panic(
            account.scrypto().as_node_id().0,
        );
        let output = RetAccessControllerRecoveryFeeOutput {
            contributions: vec![RetAccessControllerRecoveryFeeTransfer {
                access_controller: scrypto_access_controller,
                xrd_amount: 10.into(),
            }],
            withdrawals: Vec::new(),
            account_withdrawals: vec![RetAccountRecoveryFeeTransfer {
                account: scrypto_account,
                xrd_amount: 10.into(),
            }],
            account_deposits: Vec::new(),
        };

        assert_eq!(
            SUT::from((output, NetworkID::Mainnet)),
            SUT::AccessControllerRecoveryFee {
                ac_addresses: vec![access_controller],
                contributions: vec![AccessControllerXrdTransfer {
                    access_controller,
                    xrd_amount: Decimal192::ten(),
                    controlled_entity: None,
                }],
                withdrawals: Vec::new(),
                account_withdrawals: vec![AccountXrdTransfer {
                    account,
                    xrd_amount: Decimal192::ten(),
                }],
                account_deposits: Vec::new(),
            }
        );
    }
}
