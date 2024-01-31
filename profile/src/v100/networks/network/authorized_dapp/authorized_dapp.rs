use crate::prelude::*;

/// A connection made between a Radix Dapp and the user.
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizedDapp {
    #[serde(rename = "networkID")]
    pub network_id: NetworkID,

    #[serde(rename = "dAppDefinitionAddress")]
    pub dapp_definition_address: DappDefinitionAddress,

    /// Will be nil if we failed to fetch the Dapp Metadata from On-Network for some reason, and
    /// which is allowed if `isDeveloperMode: true` is set.
    pub display_name: Option<String>,

    pub references_to_authorized_personas:
        IdentifiedVecVia<AuthorizedPersonaSimple>,
}

impl AuthorizedDapp {
    pub fn new(
        network_id: NetworkID,
        dapp_definition_address: DappDefinitionAddress,
        display_name: Option<impl AsRef<str>>,
        references_to_authorized_personas: IdentifiedVecVia<
            AuthorizedPersonaSimple,
        >,
    ) -> Self {
        assert_eq!(dapp_definition_address.network_id, network_id,  "Discrepancy, found an DappDefinitionAddress on other network than {network_id}");
        assert!(references_to_authorized_personas.ids().iter().all(|i| i.network_id == network_id), "Discrepancy, found an (Authorized)Persona(Simple) on other network than {network_id}");
        Self {
            network_id,
            dapp_definition_address,
            display_name: display_name.map(|n| n.as_ref().to_string()),
            references_to_authorized_personas,
        }
    }
}

impl Identifiable for AuthorizedDapp {
    type ID = DappDefinitionAddress;

    fn id(&self) -> Self::ID {
        self.dapp_definition_address.clone()
    }
}

pub type DappDefinitionAddress = AccountAddress;

impl Identifiable for AccountAddress {
    type ID = Self;

    fn id(&self) -> Self::ID {
        self.clone()
    }
}

impl AuthorizedDapp {
    pub fn placeholder_mainnet_dashboard() -> Self {
        Self::new(
            NetworkID::Mainnet,
             "account_rdx12x0xfz2yumu2qsh6yt0v8xjfc7et04vpsz775kc3yd3xvle4w5d5k5"
             .parse()
             .expect("Valid Dapp Def Address"),
              Some("Radix Dashboard"), 
             IdentifiedVecVia::from_iter([
                    AuthorizedPersonaSimple::placeholder_mainnet(),
                    AuthorizedPersonaSimple::placeholder_mainnet_other()
                ])
            )
    }
    pub fn placeholder_mainnet_gumballclub() -> Self {
        Self::new(
            NetworkID::Mainnet,
             "account_rdx12xuhw6v30chdkhcu7qznz9vu926vxefr4h4tdvc0mdckg9rq4afx9t"
             .parse()
             .expect("Valid Dapp Def Address"),
              Some("Gumball Club"), 
             IdentifiedVecVia::from_iter([
                    AuthorizedPersonaSimple::placeholder_mainnet_other()
                ])
            )
    }
    pub fn placeholder_stokenet_devconsole() -> Self {
        Self::new(
            NetworkID::Stokenet,
             "account_tdx_2_128evrrwfp8gj9240qq0m06ukhwaj2cmejluxxreanzjwq62vmlf8r4"
             .parse()
             .expect("Valid Dapp Def Address"),
              Some("Dev Console"), 
             IdentifiedVecVia::from_iter([
                    AuthorizedPersonaSimple::placeholder_stokenet(),
                    AuthorizedPersonaSimple::placeholder_stokenet_other()
                ])
            )
    }
    pub fn placeholder_stokenet_sandbox() -> Self {
        Self::new(
            NetworkID::Stokenet,
             "account_tdx_2_12yf9gd53yfep7a669fv2t3wm7nz9zeezwd04n02a433ker8vza6rhe"
             .parse()
             .expect("Valid Dapp Def Address"),
              Some("Sandbox"), 
             IdentifiedVecVia::from_iter([
                    AuthorizedPersonaSimple::placeholder_stokenet_other()
                ])
            )
    }

    pub fn placeholder_mainnet() -> Self {
        Self::placeholder_mainnet_dashboard()
    }

    pub fn placeholder_mainnet_other() -> Self {
        Self::placeholder_mainnet_gumballclub()
    }

    pub fn placeholder_stokenet() -> Self {
        Self::placeholder_stokenet_devconsole()
    }

    pub fn placeholder_stokenet_other() -> Self {
        Self::placeholder_stokenet_sandbox()
    }
}
impl HasPlaceholder for AuthorizedDapp {
    fn placeholder() -> Self {
        Self::placeholder_mainnet_dashboard()
    }
    fn placeholder_other() -> Self {
        Self::placeholder_mainnet_gumballclub()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn json_mainnet_roundtrip() {
        let model = AuthorizedDapp::placeholder_mainnet();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
				"networkID": 1,
				"dAppDefinitionAddress": "account_rdx12x0xfz2yumu2qsh6yt0v8xjfc7et04vpsz775kc3yd3xvle4w5d5k5",
				"displayName": "Radix Dashboard",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_rdx122kttqch0eehzj6f9nkkxcw7msfeg9udurq5u0ysa0e92c59w0mg6x",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "exactly",
								"quantity": 2
							},
							"ids": [
								"account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease",
								"account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master"
							]
						}
					},
					{
						"identityAddress": "identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 1
							},
							"ids": [
								"account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master"
							]
						}
					}
				]
			}
            "#,
        );
    }

    #[test]
    fn json_mainnet_other_roundtrip() {
        let model = AuthorizedDapp::placeholder_mainnet_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
				"networkID": 1,
				"dAppDefinitionAddress": "account_rdx12xuhw6v30chdkhcu7qznz9vu926vxefr4h4tdvc0mdckg9rq4afx9t",
				"displayName": "Gumball Club",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 1
							},
							"ids": [
								"account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master"
							]
						}
					}
				]
			}
            "#,
        );
    }

    #[test]
    fn json_stokenet_roundtrip() {
        let model = AuthorizedDapp::placeholder_stokenet();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
				"networkID": 2,
				"dAppDefinitionAddress": "account_tdx_2_128evrrwfp8gj9240qq0m06ukhwaj2cmejluxxreanzjwq62vmlf8r4",
				"displayName": "Dev Console",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_tdx_2_12fk6qyu2860xyx2jk7j6ex464ccrnxrve4kpaa8qyxx99y5627ahhc",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "exactly",
								"quantity": 2
							},
							"ids": [
								"account_tdx_2_1289zm062j788dwrjefqkfgfeea5tkkdnh8htqhdrzdvjkql4kxceql",
								"account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr"
							]
						}
					},
					{
						"identityAddress": "identity_tdx_2_12gr0d9da3jvye7mdrreljyqs35esjyjsl9r8t5v96hq6fq367cln08",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 1
							},
							"ids": [
								"account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr"
							]
						}
					}
				]
			}
            "#,
        );
    }

    #[test]
    fn json_stokenet_other_roundtrip() {
        let model = AuthorizedDapp::placeholder_stokenet_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
				"networkID": 2,
				"dAppDefinitionAddress": "account_tdx_2_12yf9gd53yfep7a669fv2t3wm7nz9zeezwd04n02a433ker8vza6rhe",
				"displayName": "Sandbox",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_tdx_2_12gr0d9da3jvye7mdrreljyqs35esjyjsl9r8t5v96hq6fq367cln08",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 1
							},
							"ids": [
								"account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr"
							]
						}
					}
				]
			}
            "#,
        );
    }
}
