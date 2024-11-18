use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug, EnumAsInner)]
#[serde(tag = "type")]
pub enum StateEntityDetailsResponseItemDetails {
    FungibleResource(StateEntityDetailsResponseFungibleResourceDetails),
    NonFungibleResource(StateEntityDetailsResponseNonFungibleResourceDetails),
    FungibleVault,
    NonFungibleVault,
    Package(StateEntityDetailsResponsePackageDetails),
    Component(StateEntityDetailsResponseComponentDetails),
}

impl StateEntityDetailsResponseItemDetails {
    pub fn role_assignments(&self) -> Option<ComponentEntityRoleAssignments> {
        match self {
            Self::FungibleResource(details) => {
                Some(details.role_assignments.clone())
            }
            Self::NonFungibleResource(details) => {
                Some(details.role_assignments.clone())
            }
            Self::FungibleVault => None,
            Self::NonFungibleVault => None,
            Self::Package(details) => details.role_assignments.clone(),
            Self::Component(details) => details.role_assignments.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = StateEntityDetailsResponseItemDetails;

    #[test]
    fn json() {
        // Note: we aren't using `assert_eq_after_json_roundtrip` to verify the roundtrip because there
        // are multiple fields that we aren't parsing, so we can't compare the entire struct.

        let sut = SUT::FungibleResource(
            StateEntityDetailsResponseFungibleResourceDetails::new(
                ComponentEntityRoleAssignments::sample_allow_all(),
            ),
        );
        let json = r#"
{
  "type": "FungibleResource",
  "role_assignments": {
    "entries": [
      {
        "role_key": {
          "module": "Main",
          "name": "depositor"
        },
        "assignment": {
          "resolution": "Explicit",
          "explicit_rule": {
            "type": "AllowAll"
          }
        },
        "updater_roles": [
          {
            "module": "Main",
            "name": "depositor_updater"
          }
        ]
      },
      {
        "role_key": {
          "module": "Main",
          "name": "withdrawer"
        },
        "assignment": {
          "resolution": "Explicit",
          "explicit_rule": {
            "type": "AllowAll"
          }
        },
        "updater_roles": [
          {
            "module": "Main",
            "name": "withdrawer_updater"
          }
        ]
      }
    ]
  }
}
        "#;

        let result: SUT = serde_json::from_str(json).unwrap();
        assert_eq!(result, sut);
    }
}
