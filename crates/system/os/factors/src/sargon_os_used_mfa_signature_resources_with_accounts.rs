use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsedMfaSignatureResourceWithAccounts {
    pub mfa_factor_instance: MFAFactorInstance,
    pub non_fungible_global_id: NonFungibleGlobalId,
    pub account_addresses: Vec<AccountAddress>,
}

impl HasSampleValues for UsedMfaSignatureResourceWithAccounts {
    fn sample() -> Self {
        Self {
            mfa_factor_instance: MFAFactorInstance::sample(),
            non_fungible_global_id: NonFungibleGlobalId::sample(),
            account_addresses: vec![AccountAddress::sample()],
        }
    }

    fn sample_other() -> Self {
        Self {
            mfa_factor_instance: MFAFactorInstance::sample_other(),
            non_fungible_global_id: NonFungibleGlobalId::sample_other(),
            account_addresses: vec![AccountAddress::sample_other()],
        }
    }
}

#[async_trait::async_trait]
pub trait OsUsedMfaSignatureResourcesWithAccounts {
    async fn used_mfa_signature_resources_with_accounts_current_network(
        &self,
    ) -> Result<Vec<UsedMfaSignatureResourceWithAccounts>>;
}

#[async_trait::async_trait]
impl OsUsedMfaSignatureResourcesWithAccounts for SargonOS {
    async fn used_mfa_signature_resources_with_accounts_current_network(
        &self,
    ) -> Result<Vec<UsedMfaSignatureResourceWithAccounts>> {
        let profile = self.profile()?;
        let current_network = match profile.current_network() {
            Ok(network) => network,
            Err(CommonError::NoNetworkInProfile { .. }) => return Ok(vec![]),
            Err(error) => return Err(error),
        };
        let network_id = current_network.id;
        let mfa_factor_instances = current_network.mfa_factor_instances.clone();

        if mfa_factor_instances.is_empty() {
            return Ok(vec![]);
        }

        let mapped = mfa_factor_instances
            .into_iter()
            .map(|mfa_factor_instance| {
                let requirement = RoleRequirement::from(
                    mfa_factor_instance.clone().factor_instance.badge,
                );
                let non_fungible_global_id = NonFungibleGlobalId::from((
                    mfa_factor_instance
                        .factor_instance
                        .badge
                        .try_non_fungible_global_id()?,
                    network_id,
                ));
                Ok((mfa_factor_instance, requirement, non_fungible_global_id))
            })
            .collect::<Result<Vec<_>>>()?;

        let requirements = mapped
            .iter()
            .map(|(_, requirement, _)| requirement.clone())
            .collect_vec();

        let response = self
            .gateway_client()?
            .fetch_entities_by_role_requirement_lookup(requirements)
            .await?;

        let used =
            response
                .items
                .into_iter()
                .filter_map(|item| {
                    let (
                        mfa_factor_instance,
                        _requirement,
                        non_fungible_global_id,
                    ) = mapped
                        .iter()
                        .find(|(_, requirement, _)| {
                            *requirement == item.requirement
                        })?
                        .clone();

                    let account_addresses = item
                        .entities
                        .into_iter()
                        .filter_map(|entity| entity.entity_address)
                        .filter_map(|address| address.as_account().cloned())
                        .collect::<IndexSet<_>>()
                        .into_iter()
                        .collect_vec();

                    if account_addresses.is_empty() {
                        return None;
                    }

                    Some(UsedMfaSignatureResourceWithAccounts {
                        mfa_factor_instance,
                        non_fungible_global_id,
                        account_addresses,
                    })
                })
                .collect_vec();

        Ok(used)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn returns_empty_when_no_stored_mfa_instances() {
        let os = SUT::boot_test_with_networking_driver(Arc::new(
            MockNetworkingDriver::new_always_failing(),
        ))
        .await
        .unwrap();

        let result = os
            .used_mfa_signature_resources_with_accounts_current_network()
            .await
            .unwrap();

        assert!(result.is_empty());
    }

    #[actix_rt::test]
    async fn returns_only_used_instances_with_deduped_account_addresses() {
        let mock_driver = MockNetworkingDriver::with_lazy_response(
            |req: EntitiesByRoleRequirementLookupRequest, _count: u64| {
                let first = req.requirements.first().cloned().unwrap();
                let second = req.requirements.get(1).cloned().unwrap();

                let items = vec![
                    EntitiesByRoleRequirementLookupItem {
                        total_count: 4,
                        requirement: first,
                        entities: vec![
                            EntityByRoleRequirement {
                                entity_address: Some(Address::from(
                                    AccountAddress::sample_stokenet(),
                                )),
                                first_seen_state_version: 1,
                            },
                            EntityByRoleRequirement {
                                entity_address: Some(Address::from(
                                    IdentityAddress::sample_stokenet(),
                                )),
                                first_seen_state_version: 1,
                            },
                            EntityByRoleRequirement {
                                entity_address: Some(Address::from(
                                    AccountAddress::sample_stokenet(),
                                )),
                                first_seen_state_version: 1,
                            },
                            EntityByRoleRequirement {
                                entity_address: None,
                                first_seen_state_version: 1,
                            },
                        ],
                    },
                    EntitiesByRoleRequirementLookupItem {
                        total_count: 0,
                        requirement: second,
                        entities: vec![],
                    },
                ];
                EntitiesByRoleRequirementLookupResponse::new(items)
            },
        );

        let os = SUT::boot_test_with_networking_driver(Arc::new(mock_driver))
            .await
            .unwrap();

        os.change_current_gateway(Gateway::stokenet())
            .await
            .unwrap();

        let mfa_0 =
            MFAFactorInstance::sample_stokenet_account_securified_idx_0();
        let mfa_1 =
            MFAFactorInstance::sample_stokenet_account_securified_idx_1();
        os.update_profile_with(|profile| {
            profile.networks.insert_mfa_factor_instances(
                profile.current_network_id(),
                MFAFactorInstances::from_iter([mfa_0.clone(), mfa_1.clone()]),
            );
            Ok(())
        })
        .await
        .unwrap();

        let result = os
            .used_mfa_signature_resources_with_accounts_current_network()
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].mfa_factor_instance, mfa_0);
        assert_eq!(
            result[0].account_addresses,
            vec![AccountAddress::sample_stokenet()]
        );
    }

    #[actix_rt::test]
    async fn derives_non_fungible_global_id_from_instance_badge_and_network_id()
    {
        let mock_driver = MockNetworkingDriver::with_lazy_response(
            |req: EntitiesByRoleRequirementLookupRequest, _count: u64| {
                let items = req
                    .requirements
                    .into_iter()
                    .map(|requirement| EntitiesByRoleRequirementLookupItem {
                        total_count: 1,
                        requirement,
                        entities: vec![EntityByRoleRequirement {
                            entity_address: Some(Address::from(
                                AccountAddress::sample_stokenet(),
                            )),
                            first_seen_state_version: 1,
                        }],
                    })
                    .collect_vec();

                EntitiesByRoleRequirementLookupResponse::new(items)
            },
        );

        let os = SUT::boot_test_with_networking_driver(Arc::new(mock_driver))
            .await
            .unwrap();

        os.change_current_gateway(Gateway::stokenet())
            .await
            .unwrap();

        let mfa = MFAFactorInstance::sample_stokenet_account_securified_idx_0();
        os.update_profile_with(|profile| {
            profile.networks.insert_mfa_factor_instances(
                profile.current_network_id(),
                MFAFactorInstances::just(mfa.clone()),
            );
            Ok(())
        })
        .await
        .unwrap();

        let result = os
            .used_mfa_signature_resources_with_accounts_current_network()
            .await
            .unwrap();

        assert_eq!(result.len(), 1);

        let expected = NonFungibleGlobalId::from((
            mfa.factor_instance
                .badge
                .try_non_fungible_global_id()
                .unwrap(),
            NetworkID::Stokenet,
        ));
        assert_eq!(result[0].non_fungible_global_id, expected);
    }
}
