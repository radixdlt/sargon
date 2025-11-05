use crate::prelude::*;

/// JSON model of the Access Controller state substate value.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct AccessControllerFieldStateValue {
    pub controlled_vault: EntityReference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xrd_fee_vault: Option<EntityReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_recovery_delay_minutes: Option<u32>,
    pub recovery_badge_resource_address: ResourceAddress,
    pub is_primary_role_locked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_role_recovery_attempt: Option<PrimaryRoleRecoveryAttempt>,
    pub has_primary_role_badge_withdraw_attempt: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_role_recovery_attempt: Option<RecoveryRoleRecoveryAttempt>,
    pub has_recovery_role_badge_withdraw_attempt: bool,
}

impl HasSampleValues for AccessControllerFieldStateValue {
    fn sample() -> Self {
        Self {
            controlled_vault: EntityReference {
                entity_type: CoreApiEntityType::InternalNonFungibleVault,
                is_global: false,
                entity_address: "internal_vault_rdx1nqutf8slj3qyasr8jepflggtycdxjzvnkawm8zahmax5xs0eetyehf".to_string(),
            },
            xrd_fee_vault: None,
            timed_recovery_delay_minutes: Some(20160),
            recovery_badge_resource_address: "resource_rdx1nfs0la98ht4fz0pjkd62ep0uulmljwdhztsfvehz0gkwp3drqu2hyx".parse().unwrap(),
            is_primary_role_locked: false,
            primary_role_recovery_attempt: None,
            has_primary_role_badge_withdraw_attempt: false,
            recovery_role_recovery_attempt: None,
            has_recovery_role_badge_withdraw_attempt: false,
        }
    }

    fn sample_other() -> Self {
        Self {
            controlled_vault: EntityReference {
                entity_type: CoreApiEntityType::InternalNonFungibleVault,
                is_global: false,
                entity_address: "internal_vault_tdx_2_1nrs3a0qw5qfsx83hvkn8h6l6pfsc703gr8k5wz24gup97zxjsct0t7".to_string(),
            },
            xrd_fee_vault: Some(EntityReference {
                entity_type: CoreApiEntityType::InternalNonFungibleVault,
                is_global: false,
                entity_address: "internal_vault_tdx_2_1ny42z9ly8zw4e8wqzkcazqthq52j8qre6xtydsz7qgkzy8wp3v48sp".to_string(),
            }),
            timed_recovery_delay_minutes: Some(120),
            recovery_badge_resource_address: "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc".parse().unwrap(),
            is_primary_role_locked: true,
            primary_role_recovery_attempt: Some(PrimaryRoleRecoveryAttempt {
                recovery_proposal: RecoveryProposal {
                    primary_role: AccessRule::AllowAll,
                    recovery_role: AccessRule::Protected {
                        access_rule: CompositeRequirement::AnyOf { access_rules: vec![] },
                    },
                    confirmation_role: AccessRule::Protected {
                        access_rule: CompositeRequirement::ProofRule {
                            proof_rule: BasicRequirement::Require {
                                requirement: Requirement::Resource {
                                    resource: "resource_tdx_2_1ng6aanl0nw98dgqxtja3mx4kpa8rzwhyt4q22sy9uul0vf9frs528x".parse().unwrap(),
                                },
                            },
                        },
                    },
                    timed_recovery_delay_minutes: Some(45),
                },
            }),
            has_primary_role_badge_withdraw_attempt: true,
            recovery_role_recovery_attempt: Some(RecoveryRoleRecoveryAttempt {
                recovery_proposal: RecoveryProposal {
                    primary_role: AccessRule::DenyAll,
                    recovery_role: AccessRule::AllowAll,
                    confirmation_role: AccessRule::AllowAll,
                    timed_recovery_delay_minutes: None,
                },
                allow_timed_recovery_after: Some(ScryptoInstantDto {
                    unix_timestamp_seconds: "1730999831".to_string(),
                    date_time: Some("2024-11-07T11:17:11Z".to_string()),
                }),
            }),
            has_recovery_role_badge_withdraw_attempt: true,
        }
    }
}

/// Reference to an entity (global or internal) returned by the Core API.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct EntityReference {
    pub entity_type: CoreApiEntityType,
    pub is_global: bool,
    pub entity_address: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub enum CoreApiEntityType {
    #[serde(rename = "GlobalPackage")]
    GlobalPackage,
    #[serde(rename = "GlobalConsensusManager")]
    GlobalConsensusManager,
    #[serde(rename = "GlobalValidator")]
    GlobalValidator,
    #[serde(rename = "GlobalGenericComponent")]
    GlobalGenericComponent,
    #[serde(rename = "GlobalAccount")]
    GlobalAccount,
    #[serde(rename = "GlobalIdentity")]
    GlobalIdentity,
    #[serde(rename = "GlobalAccessController")]
    GlobalAccessController,
    #[serde(rename = "GlobalVirtualSecp256k1Account")]
    GlobalVirtualSecp256k1Account,
    #[serde(rename = "GlobalVirtualSecp256k1Identity")]
    GlobalVirtualSecp256k1Identity,
    #[serde(rename = "GlobalVirtualEd25519Account")]
    GlobalVirtualEd25519Account,
    #[serde(rename = "GlobalVirtualEd25519Identity")]
    GlobalVirtualEd25519Identity,
    #[serde(rename = "GlobalFungibleResource")]
    GlobalFungibleResource,
    #[serde(rename = "InternalFungibleVault")]
    InternalFungibleVault,
    #[serde(rename = "GlobalNonFungibleResource")]
    GlobalNonFungibleResource,
    #[serde(rename = "InternalNonFungibleVault")]
    InternalNonFungibleVault,
    #[serde(rename = "InternalGenericComponent")]
    InternalGenericComponent,
    #[serde(rename = "InternalKeyValueStore")]
    InternalKeyValueStore,
    #[serde(rename = "GlobalOneResourcePool")]
    GlobalOneResourcePool,
    #[serde(rename = "GlobalTwoResourcePool")]
    GlobalTwoResourcePool,
    #[serde(rename = "GlobalMultiResourcePool")]
    GlobalMultiResourcePool,
    #[serde(rename = "GlobalTransactionTracker")]
    GlobalTransactionTracker,
    #[serde(rename = "GlobalAccountLocker")]
    GlobalAccountLocker,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct PrimaryRoleRecoveryAttempt {
    pub recovery_proposal: RecoveryProposal,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct RecoveryRoleRecoveryAttempt {
    pub recovery_proposal: RecoveryProposal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_timed_recovery_after: Option<ScryptoInstantDto>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct RecoveryProposal {
    pub primary_role: AccessRule,
    pub recovery_role: AccessRule,
    pub confirmation_role: AccessRule,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_recovery_delay_minutes: Option<u32>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
pub enum AccessRule {
    #[serde(rename = "Protected")]
    Protected { access_rule: CompositeRequirement },
    #[serde(rename = "AllowAll")]
    AllowAll,
    #[serde(rename = "DenyAll")]
    DenyAll,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
pub enum CompositeRequirement {
    #[serde(rename = "ProofRule")]
    ProofRule { proof_rule: BasicRequirement },
    #[serde(rename = "AnyOf")]
    AnyOf {
        access_rules: Vec<CompositeRequirement>,
    },
    #[serde(rename = "AllOf")]
    AllOf {
        access_rules: Vec<CompositeRequirement>,
    },
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
pub enum BasicRequirement {
    #[serde(rename = "Require")]
    Require { requirement: Requirement },
    #[serde(rename = "AmountOf")]
    AmountOf {
        amount: Decimal192,
        resource: ResourceAddress,
    },
    #[serde(rename = "AllOf")]
    AllOf { list: Vec<Requirement> },
    #[serde(rename = "AnyOf")]
    AnyOf { list: Vec<Requirement> },
    #[serde(rename = "CountOf")]
    CountOf { count: u32, list: Vec<Requirement> },
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
pub enum Requirement {
    #[serde(rename = "Resource")]
    Resource { resource: ResourceAddress },
    #[serde(rename = "NonFungible")]
    NonFungible { non_fungible: NonFungible },
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct NonFungible {
    pub resource_address: ResourceAddress,
    pub local_id: NonFungibleLocalIdId,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct NonFungibleLocalIdId {
    pub id_type: String,
    pub sbor_hex: String,
    pub simple_rep: String,
}

/// DTO for the `ScryptoInstant` type from the Core API schema.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct ScryptoInstantDto {
    pub unix_timestamp_seconds: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_minimal_payload() {
        let json = r#"
        {
            "controlled_vault": {
                "entity_type": "InternalNonFungibleVault",
                "is_global": false,
                "entity_address": "internal_vault_rdx1nqutf8slj3qyasr8jepflggtycdxjzvnkawm8zahmax5xs0eetyehf"
            },
            "timed_recovery_delay_minutes": 20160,
            "recovery_badge_resource_address": "resource_rdx1nfs0la98ht4fz0pjkd62ep0uulmljwdhztsfvehz0gkwp3drqu2hyx",
            "is_primary_role_locked": false,
            "has_primary_role_badge_withdraw_attempt": false,
            "has_recovery_role_badge_withdraw_attempt": false
        }
        "#;

        let decoded: AccessControllerFieldStateValue =
            serde_json::from_str(json).unwrap();

        let expected = AccessControllerFieldStateValue {
            controlled_vault: EntityReference {
                entity_type: CoreApiEntityType::InternalNonFungibleVault,
                is_global: false,
                entity_address: "internal_vault_rdx1nqutf8slj3qyasr8jepflggtycdxjzvnkawm8zahmax5xs0eetyehf"
                    .to_string(),
            },
            xrd_fee_vault: None,
            timed_recovery_delay_minutes: Some(20160),
            recovery_badge_resource_address: "resource_rdx1nfs0la98ht4fz0pjkd62ep0uulmljwdhztsfvehz0gkwp3drqu2hyx"
                .parse()
                .unwrap(),
            is_primary_role_locked: false,
            primary_role_recovery_attempt: None,
            has_primary_role_badge_withdraw_attempt: false,
            recovery_role_recovery_attempt: None,
            has_recovery_role_badge_withdraw_attempt: false,
        };

        assert_eq!(decoded, expected);
    }

    #[test]
    fn deserialize_full_payload() {
        let json = r#"
        {
            "controlled_vault": {
                "entity_type": "InternalFungibleVault",
                "is_global": false,
                "entity_address": "internal_vault_tdx_2_1nrs3a0qw5qfsx83hvkn8h6l6pfsc703gr8k5wz24gup97zxjsct0t7"
            },
            "xrd_fee_vault": {
                "entity_type": "InternalFungibleVault",
                "is_global": false,
                "entity_address": "internal_vault_tdx_2_1ny42z9ly8zw4e8wqzkcazqthq52j8qre6xtydsz7qgkzy8wp3v48sp"
            },
            "timed_recovery_delay_minutes": 120,
            "recovery_badge_resource_address": "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc",
            "is_primary_role_locked": true,
            "primary_role_recovery_attempt": {
                "recovery_proposal": {
                    "primary_role": { "type": "AllowAll" },
                    "recovery_role": {
                        "type": "Protected",
                        "access_rule": {
                            "type": "AnyOf",
                            "access_rules": []
                        }
                    },
                    "confirmation_role": {
                        "type": "Protected",
                        "access_rule": {
                            "type": "ProofRule",
                            "proof_rule": {
                                "type": "Require",
                                "requirement": {
                                    "type": "Resource",
                                    "resource": "resource_tdx_2_1ng6aanl0nw98dgqxtja3mx4kpa8rzwhyt4q22sy9uul0vf9frs528x"
                                }
                            }
                        }
                    },
                    "timed_recovery_delay_minutes": 45
                }
            },
            "has_primary_role_badge_withdraw_attempt": true,
            "recovery_role_recovery_attempt": {
                "recovery_proposal": {
                    "primary_role": { "type": "DenyAll" },
                    "recovery_role": { "type": "AllowAll" },
                    "confirmation_role": { "type": "AllowAll" }
                },
                "allow_timed_recovery_after": {
                    "unix_timestamp_seconds": "1730999831",
                    "date_time": "2024-11-07T11:17:11Z"
                }
            },
            "has_recovery_role_badge_withdraw_attempt": true
        }
        "#;

        let decoded: AccessControllerFieldStateValue =
            serde_json::from_str(json).unwrap();

        let expected = AccessControllerFieldStateValue {
            controlled_vault: EntityReference {
                entity_type: CoreApiEntityType::InternalFungibleVault,
                is_global: false,
                entity_address: "internal_vault_tdx_2_1nrs3a0qw5qfsx83hvkn8h6l6pfsc703gr8k5wz24gup97zxjsct0t7"
                    .to_string(),
            },
            xrd_fee_vault: Some(EntityReference {
                entity_type: CoreApiEntityType::InternalFungibleVault,
                is_global: false,
                entity_address: "internal_vault_tdx_2_1ny42z9ly8zw4e8wqzkcazqthq52j8qre6xtydsz7qgkzy8wp3v48sp"
                    .to_string(),
            }),
            timed_recovery_delay_minutes: Some(120),
            recovery_badge_resource_address: "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc"
                .parse()
                .unwrap(),
            is_primary_role_locked: true,
            primary_role_recovery_attempt: Some(PrimaryRoleRecoveryAttempt {
                recovery_proposal: RecoveryProposal {
                    primary_role: AccessRule::AllowAll,
                    recovery_role: AccessRule::Protected {
                        access_rule: CompositeRequirement::AnyOf {
                            access_rules: vec![],
                        },
                    },
                    confirmation_role: AccessRule::Protected {
                        access_rule: CompositeRequirement::ProofRule {
                            proof_rule: BasicRequirement::Require {
                                requirement: Requirement::Resource {
                                    resource: "resource_tdx_2_1ng6aanl0nw98dgqxtja3mx4kpa8rzwhyt4q22sy9uul0vf9frs528x"
                                        .parse()
                                        .unwrap(),
                                },
                            },
                        },
                    },
                    timed_recovery_delay_minutes: Some(45),
                },
            }),
            has_primary_role_badge_withdraw_attempt: true,
            recovery_role_recovery_attempt: Some(RecoveryRoleRecoveryAttempt {
                recovery_proposal: RecoveryProposal {
                    primary_role: AccessRule::DenyAll,
                    recovery_role: AccessRule::AllowAll,
                    confirmation_role: AccessRule::AllowAll,
                    timed_recovery_delay_minutes: None,
                },
                allow_timed_recovery_after: Some(ScryptoInstantDto {
                    unix_timestamp_seconds: "1730999831".to_string(),
                    date_time: Some("2024-11-07T11:17:11Z".to_string()),
                }),
            }),
            has_recovery_role_badge_withdraw_attempt: true,
        };

        assert_eq!(decoded, expected);
    }
}
