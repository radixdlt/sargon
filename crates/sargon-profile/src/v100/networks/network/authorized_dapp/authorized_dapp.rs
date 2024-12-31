use crate::prelude::*;

/// A connection made between a Radix Dapp and the user.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
)]
#[serde(rename_all = "camelCase")]
#[display("{}", self.description())]
pub struct AuthorizedDapp {
    /// The ID of the network the authorized Dapp is on.
    #[serde(rename = "networkID")]
    pub network_id: NetworkID,

    /// A `DappDefinitionAddress` is in fact just an alias for
    /// [`AccountAddress`], it is the address of the account
    /// which owns controls the Dapp.
    #[serde(rename = "dAppDefinitionAddress")]
    pub dapp_definition_address: DappDefinitionAddress,

    /// The Display name as sent by the Dapp in any interaction
    /// request (CAP21), e.g. "Radix Dashboard".
    pub display_name: Option<String>,

    /// An order set of `AuthorizedPersonaSimple`s, which is a collection of all
    /// the Personas the user has used to interact with this Dapp, it is called
    /// "references to", since the Personas are not stored in full, that would be
    /// bad duplication of data (which might go stale), instead we refer to the
    /// necessary data by IDs.
    pub references_to_authorized_personas: ReferencesToAuthorizedPersonas,

    /// The preferences the user has configured for this Dapp.
    #[serde(default)]
    pub preferences: AuthorizedDappPreferences,
}

impl IsNetworkAware for AuthorizedDapp {
    fn network_id(&self) -> NetworkID {
        self.network_id
    }
}

impl AuthorizedDapp {
    pub fn description(&self) -> String {
        format!(
            r#"
			network_id: {}
			dapp_definition_address: {}
			display_name: {}
			references_to_authorized_personas: {}
			preferences: {}
			"#,
            self.network_id,
            self.dapp_definition_address,
            self.display_name.clone().unwrap_or("<NONE>".to_owned()),
            self.references_to_authorized_personas,
            self.preferences,
        )
    }

    pub fn new(
        network_id: NetworkID,
        dapp_definition_address: DappDefinitionAddress,
        display_name: impl Into<Option<String>>,
        references_to_authorized_personas: ReferencesToAuthorizedPersonas,
        preferences: AuthorizedDappPreferences,
    ) -> Self {
        assert_eq!(dapp_definition_address.network_id(), network_id,  "Discrepancy, found an DappDefinitionAddress on other network than {network_id}");
        assert!(references_to_authorized_personas.ids().iter().all(|i| i.network_id() == network_id), "Discrepancy, found an (Authorized)Persona(Simple) on other network than {network_id}");
        Self {
            network_id,
            dapp_definition_address,
            display_name: display_name.into(),
            references_to_authorized_personas,
            preferences,
        }
    }
}

impl Identifiable for AuthorizedDapp {
    type ID = DappDefinitionAddress;

    fn id(&self) -> Self::ID {
        self.dapp_definition_address
    }
}

pub type DappDefinitionAddress = AccountAddress;

impl AuthorizedDapp {
    pub fn sample_mainnet_dashboard() -> Self {
        Self::new(
            NetworkID::Mainnet,
            "account_rdx12x0xfz2yumu2qsh6yt0v8xjfc7et04vpsz775kc3yd3xvle4w5d5k5"
             .parse()
             .expect("Valid Dapp Def Address"),
            "Radix Dashboard".to_owned(), 
			ReferencesToAuthorizedPersonas::from_iter([
                    AuthorizedPersonaSimple::sample_mainnet(),
                    AuthorizedPersonaSimple::sample_mainnet_other()
                ]),
			AuthorizedDappPreferences::sample(),
        )
    }
    pub fn sample_mainnet_gumballclub() -> Self {
        Self::new(
            NetworkID::Mainnet,
            "account_rdx12xuhw6v30chdkhcu7qznz9vu926vxefr4h4tdvc0mdckg9rq4afx9t"
             .parse()
             .expect("Valid Dapp Def Address"),
            "Gumball Club".to_owned(), 
			ReferencesToAuthorizedPersonas::from_iter([
                    AuthorizedPersonaSimple::sample_mainnet_other()
                ]),
            AuthorizedDappPreferences::sample(),
		)
    }
    pub fn sample_stokenet_devconsole() -> Self {
        Self::new(
            NetworkID::Stokenet,
            "account_tdx_2_128evrrwfp8gj9240qq0m06ukhwaj2cmejluxxreanzjwq62vmlf8r4"
             .parse()
             .expect("Valid Dapp Def Address"),
            "Dev Console".to_owned(), 
			ReferencesToAuthorizedPersonas::from_iter([
                    AuthorizedPersonaSimple::sample_stokenet(),
                    AuthorizedPersonaSimple::sample_stokenet_other()
                ]),
			AuthorizedDappPreferences::sample(),
        )
    }
    pub fn sample_stokenet_sandbox() -> Self {
        Self::new(
            NetworkID::Stokenet,
            "account_tdx_2_12yf9gd53yfep7a669fv2t3wm7nz9zeezwd04n02a433ker8vza6rhe"
             .parse()
             .expect("Valid Dapp Def Address"),
            "Sandbox".to_owned(), 
			ReferencesToAuthorizedPersonas::from_iter([
                    AuthorizedPersonaSimple::sample_stokenet_other()
                ]),
			AuthorizedDappPreferences::sample(),
        )
    }

    pub fn sample_mainnet() -> Self {
        Self::sample_mainnet_dashboard()
    }

    pub fn sample_mainnet_other() -> Self {
        Self::sample_mainnet_gumballclub()
    }

    pub fn sample_stokenet() -> Self {
        Self::sample_stokenet_devconsole()
    }

    pub fn sample_stokenet_other() -> Self {
        Self::sample_stokenet_sandbox()
    }
}
impl HasSampleValues for AuthorizedDapp {
    fn sample() -> Self {
        Self::sample_mainnet_dashboard()
    }
    fn sample_other() -> Self {
        Self::sample_mainnet_gumballclub()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthorizedDapp;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn test_is_network_aware() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
    }

    #[test]
    fn json_mainnet_roundtrip() {
        let model = SUT::sample_mainnet();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
				"networkID": 1,
				"dAppDefinitionAddress": "account_rdx12x0xfz2yumu2qsh6yt0v8xjfc7et04vpsz775kc3yd3xvle4w5d5k5",
				"displayName": "Radix Dashboard",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "exactly",
								"quantity": 2
							},
							"ids": [
								"account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87",
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
							]
						},
						"sharedPersonaData": {
							"name": "00000000-0000-0000-0000-000000000000",
							"emailAddresses": {
								"request": {
									"quantifier": "exactly",
									"quantity": 2
								},
								"ids": [
									"00000000-0000-0000-0000-000000000003",
									"00000000-0000-0000-0000-000000000004"
								]
							},
							"phoneNumbers": {
								"request": {
									"quantifier": "exactly",
									"quantity": 2
								},
								"ids": [
									"00000000-0000-0000-0000-000000000001",
									"00000000-0000-0000-0000-000000000002"
								]
							}
						}
					},
					{
						"identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 1
							},
							"ids": [
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
							]
						},
						"sharedPersonaData": {
							"name": "00000000-0000-0000-0000-000000000000",
							"emailAddresses": {
								"request": {
									"quantifier": "exactly",
									"quantity": 1
								},
								"ids": [
									"00000000-0000-0000-0000-000000000002"
								]
							},
							"phoneNumbers": {
								"request": {
									"quantifier": "exactly",
									"quantity": 1
								},
								"ids": [
									"00000000-0000-0000-0000-000000000001"
								]
							}
						}
					}
				]
			}
            "#,
        );
    }

    #[test]
    fn json_mainnet_other_roundtrip() {
        let model = SUT::sample_mainnet_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
				"networkID": 1,
				"dAppDefinitionAddress": "account_rdx12xuhw6v30chdkhcu7qznz9vu926vxefr4h4tdvc0mdckg9rq4afx9t",
				"displayName": "Gumball Club",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 1
							},
							"ids": [
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
							]
						},
						"sharedPersonaData": {
							"name": "00000000-0000-0000-0000-000000000000",
							"emailAddresses": {
								"request": {
									"quantifier": "exactly",
									"quantity": 1
								},
								"ids": [
									"00000000-0000-0000-0000-000000000002"
								]
							},
							"phoneNumbers": {
								"request": {
									"quantifier": "exactly",
									"quantity": 1
								},
								"ids": [
									"00000000-0000-0000-0000-000000000001"
								]
							}
						}
					}
				]
			}
            "#,
        );
    }

    #[test]
    fn json_stokenet_roundtrip() {
        let model = SUT::sample_stokenet();
        print_json(&model);
        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
				"networkID": 2,
				"dAppDefinitionAddress": "account_tdx_2_128evrrwfp8gj9240qq0m06ukhwaj2cmejluxxreanzjwq62vmlf8r4",
				"displayName": "Dev Console",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_tdx_2_122r7248dkyjwt2kxf36de26w7htdwpzsm3lyjr4p0nvrgwn025dds8",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "exactly",
								"quantity": 2
							},
							"ids": [
								"account_tdx_2_128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4pqn48m",
								"account_tdx_2_12xvlee7xtg7dx599yv69tzkpeqzn4wr2nlnn3gpsm0zu0v9luqdpnp"
							]
						},
						"sharedPersonaData": {
							"name": "00000000-0000-0000-0000-000000000000",
							"emailAddresses": {
								"request": {
									"quantifier": "exactly",
									"quantity": 1
								},
								"ids": [
									"00000000-0000-0000-0000-000000000002"
								]
							},
							"phoneNumbers": {
								"request": {
									"quantifier": "exactly",
									"quantity": 1
								},
								"ids": [
									"00000000-0000-0000-0000-000000000001"
								]
							}
						}
					},
					{
						"identityAddress": "identity_tdx_2_12tltwh00wvvur4yymv63pwhhwhjzvu4za2fy7vnyue36v5dtq3pgvq",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 1
							},
							"ids": [
								"account_tdx_2_12xvlee7xtg7dx599yv69tzkpeqzn4wr2nlnn3gpsm0zu0v9luqdpnp"
							]
						},
						"sharedPersonaData": {
							"name": "00000000-0000-0000-0000-000000000000",
							"emailAddresses": {
								"request": {
									"quantifier": "exactly",
									"quantity": 1
								},
								"ids": [
									"00000000-0000-0000-0000-000000000002"
								]
							},
							"phoneNumbers": {
								"request": {
									"quantifier": "exactly",
									"quantity": 1
								},
								"ids": [
									"00000000-0000-0000-0000-000000000001"
								]
							}
						}
					}
				]
			}
            "#,
        );
    }

    #[test]
    fn json_stokenet_other_roundtrip() {
        let model = SUT::sample_stokenet_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
				"networkID": 2,
				"dAppDefinitionAddress": "account_tdx_2_12yf9gd53yfep7a669fv2t3wm7nz9zeezwd04n02a433ker8vza6rhe",
				"displayName": "Sandbox",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_tdx_2_12tltwh00wvvur4yymv63pwhhwhjzvu4za2fy7vnyue36v5dtq3pgvq",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 1
							},
							"ids": [
								"account_tdx_2_12xvlee7xtg7dx599yv69tzkpeqzn4wr2nlnn3gpsm0zu0v9luqdpnp"
							]
						},
						"sharedPersonaData": {
							"name": "00000000-0000-0000-0000-000000000000",
							"emailAddresses": {
								"request": {
									"quantifier": "exactly",
									"quantity": 1
								},
								"ids": [
									"00000000-0000-0000-0000-000000000002"
								]
							},
							"phoneNumbers": {
								"request": {
									"quantifier": "exactly",
									"quantity": 1
								},
								"ids": [
									"00000000-0000-0000-0000-000000000001"
								]
							}
						}
					}
				]
			}
            "#,
        );
    }
}
