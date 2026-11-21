use crate::prelude::*;

/// The access controller operations a pre-authorization may make. For now
/// only recoveries; others, such as confirming or stopping a timed recovery,
/// are to be added as the wallet supports them.
fn is_allowed_in_pre_authorizations(class: &DetailedManifestClass) -> bool {
    matches!(
        class,
        DetailedManifestClass::AccessControllerRecovery { .. }
    )
}

/// Whether `auth_zone` alone passes any role of any of `access_controllers`,
/// with their current `rule_sets`. An access controller without known rules
/// counts as passed.
fn can_pass_any_role_alone(
    access_controllers: &IndexSet<AccessControllerAddress>,
    rule_sets: &IndexMap<AccessControllerAddress, AccessControllerRuleSet>,
    auth_zone: &AuthZone,
) -> bool {
    access_controllers.iter().any(|access_controller| {
        rule_sets.get(access_controller).is_none_or(|rule_set| {
            [
                &rule_set.primary_role,
                &rule_set.recovery_role,
                &rule_set.confirmation_role,
            ]
            .into_iter()
            .any(|role| role.is_satisfied_by(auth_zone))
        })
    })
}

/// What the user can prove alone in a pre-authorization: the signatures of
/// the keys it is signed with, see [`OsPreAuthorizationSigningKeys`].
fn auth_zone_of(keys: Vec<PreAuthorizationSigningKey>) -> AuthZone {
    AuthZone {
        implicit_non_fungibles: keys.into_iter().map(|key| key.badge).collect(),
        proofs: Vec::new(),
    }
}

#[async_trait::async_trait]
pub(crate) trait OsValidatePreAuthAccessControllerCalls {
    async fn validate_pre_auth_access_controller_calls(
        &self,
        manifest: &SubintentManifest,
        summary: &ManifestSummary,
        execution_summary: &ExecutionSummary,
    ) -> Result<()>;
}

#[async_trait::async_trait]
impl OsValidatePreAuthAccessControllerCalls for SargonOS {
    /// Access controller methods are reserved: a dApp may call them in a
    /// pre-authorization only on an access controller the user controls
    /// together with others, never one they control alone. So the calls are
    /// allowed only when:
    /// * the manifest is classified as an access controller operation allowed
    ///   in pre-authorizations, see [`is_allowed_in_pre_authorizations`], so
    ///   it makes no other calls;
    /// * no account or persona needs to authorize it, so the keys below are
    ///   the only authority the user adds;
    /// * the keys the pre-authorization is signed with alone pass no role of
    ///   the access controllers called. Whether an access controller's
    ///   entity is in the user's Profile doesn't matter: a shared account
    ///   can be there too.
    ///
    /// Errors with the reserved instructions otherwise.
    async fn validate_pre_auth_access_controller_calls(
        &self,
        manifest: &SubintentManifest,
        summary: &ManifestSummary,
        execution_summary: &ExecutionSummary,
    ) -> Result<()> {
        let reject = |reason: &str| {
            warn!("Pre-auth access controller calls rejected: {reason}");
            Err(CommonError::ReservedInstructionsNotAllowedInManifest {
                reserved_instructions: summary
                    .reserved_instructions
                    .iter()
                    .map(|i| i.to_string())
                    .collect(),
            })
        };

        if !execution_summary
            .detailed_classification
            .as_ref()
            .is_some_and(is_allowed_in_pre_authorizations)
        {
            return reject(
                "not classified as an access controller operation allowed in pre-authorizations",
            );
        }
        if !summary.addresses_of_accounts_requiring_auth.is_empty()
            || !summary.addresses_of_personas_requiring_auth.is_empty()
        {
            return reject("an account or persona needs to authorize it");
        }
        let access_controllers = summary
            .access_controllers_called
            .iter()
            .copied()
            .collect::<IndexSet<_>>();

        let network_id = manifest.network_id();
        // An access controller whose rules can't be read is left out, which
        // counts as the user passing its roles alone.
        let rule_sets = self
            .gateway_client()
            .fetch_access_controllers_details(
                access_controllers.iter().copied().collect(),
            )
            .await?
            .into_iter()
            .filter_map(|details| {
                let rule_set = details.rule_set().ok()?;
                Some((details.address, (rule_set, network_id).into()))
            })
            .collect();
        let auth_zone = auth_zone_of(self.pre_authorization_signing_keys()?);
        if can_pass_any_role_alone(&access_controllers, &rule_sets, &auth_zone)
        {
            return reject("the user's signing keys alone pass a role");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use radix_engine_interface::prelude::{require, require_n_of, AccessRule};
    use ScryptoNonFungibleGlobalId as Badge;

    const NETWORK: NetworkID = NetworkID::Stokenet;

    fn badge(byte: u8) -> Badge {
        Badge::from_public_key(ScryptoEd25519PublicKey([byte; 32]))
    }

    /// The user's badge is 1, the others' are 2 and 3.
    fn user_alone() -> AuthZone {
        AuthZone {
            implicit_non_fungibles: [NonFungibleGlobalId::from((
                badge(1),
                NETWORK,
            ))]
            .into(),
            proofs: Vec::new(),
        }
    }

    fn shared() -> AccessRule {
        AccessRule::Protected(require_n_of(
            2u8,
            vec![badge(1), badge(2), badge(3)],
        ))
    }

    fn rule_sets(
        roles: [AccessRule; 3],
    ) -> IndexMap<AccessControllerAddress, AccessControllerRuleSet> {
        let [primary_role, recovery_role, confirmation_role] = roles;
        IndexMap::from([(
            AccessControllerAddress::sample_stokenet(),
            (
                ScryptoRuleSet {
                    primary_role,
                    recovery_role,
                    confirmation_role,
                },
                NETWORK,
            )
                .into(),
        )])
    }

    fn called() -> IndexSet<AccessControllerAddress> {
        IndexSet::from([AccessControllerAddress::sample_stokenet()])
    }

    #[test]
    fn roles_shared_with_others_cannot_be_passed_alone() {
        assert!(!can_pass_any_role_alone(
            &called(),
            &rule_sets([shared(), shared(), shared()]),
            &user_alone()
        ));
    }

    #[test]
    fn any_role_the_user_holds_alone_is_passed_alone() {
        let alone = || AccessRule::Protected(require(badge(1)));
        for roles in [
            [alone(), shared(), shared()],
            [shared(), alone(), shared()],
            [shared(), shared(), alone()],
        ] {
            assert!(can_pass_any_role_alone(
                &called(),
                &rule_sets(roles),
                &user_alone()
            ));
        }
    }

    /// Sargon's override-only roles pass with no proof at all.
    #[test]
    fn a_role_anyone_passes_is_passed_alone() {
        assert!(can_pass_any_role_alone(
            &called(),
            &rule_sets([shared(), AccessRule::AllowAll, shared()]),
            &user_alone()
        ));
    }

    #[test]
    fn an_access_controller_without_known_rules_counts_as_passed_alone() {
        assert!(can_pass_any_role_alone(
            &called(),
            &IndexMap::new(),
            &user_alone()
        ));
    }
}
