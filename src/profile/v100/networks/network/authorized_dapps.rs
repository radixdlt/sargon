use crate::prelude::*;

impl AuthorizedDapps {
    /// Returns a reference to the AuthorizedDapp identified by `address`, if it exists.
    pub fn get_authorized_dapp_by_address(
        &self,
        address: &DappDefinitionAddress,
    ) -> Option<&AuthorizedDapp> {
        self.get(address)
    }

    /// Returns references to **all** AuthorizedDapps, including hidden ones.
    pub fn get_all(&self) -> Vec<&AuthorizedDapp> {
        self.elements()
    }
}

impl HasSampleValues for AuthorizedDapps {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::sample_stokenet()
    }
}

impl AuthorizedDapps {
    /// A sample used to facilitate unit tests.
    pub fn sample_mainnet() -> Self {
        Self::with_authorized_dapps([
            AuthorizedDapp::sample_mainnet_dashboard(),
            AuthorizedDapp::sample_mainnet_gumballclub(),
        ])
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_stokenet() -> Self {
        Self::with_authorized_dapps([
            AuthorizedDapp::sample_stokenet_devconsole(),
            AuthorizedDapp::sample_stokenet_sandbox(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn default_is_empty() {
        assert_eq!(AuthorizedDapps::default().len(), 0);
    }

    #[test]
    fn inequality() {
        assert_ne!(AuthorizedDapps::sample(), AuthorizedDapps::sample_other());
    }

    #[test]
    fn equality() {
        assert_eq!(AuthorizedDapps::sample(), AuthorizedDapps::sample());
        assert_eq!(
            AuthorizedDapps::sample_other(),
            AuthorizedDapps::sample_other()
        );
    }

    #[test]
    fn duplicates_are_prevented() {
        assert_eq!(
            AuthorizedDapps::with_authorized_dapps(
                [AuthorizedDapp::sample(), AuthorizedDapp::sample()]
                    .into_iter()
            )
            .len(),
            1
        )
    }

    #[test]
    fn with_one() {
        assert_eq!(
            AuthorizedDapps::with_authorized_dapp(AuthorizedDapp::sample())
                .len(),
            1
        )
    }

    #[test]
    fn get_all() {
        assert_eq!(AuthorizedDapps::sample().get_all().len(), 2);
    }

    #[test]
    fn get_by_address() {
        let authorized_dapp = AuthorizedDapp::sample();
        let address = authorized_dapp.dapp_definition_address;
        let authorized_dapps =
            AuthorizedDapps::with_authorized_dapp(authorized_dapp.clone());
        assert_eq!(
            authorized_dapps.get_authorized_dapp_by_address(&address),
            Some(&authorized_dapp)
        );
    }

    #[test]
    fn json_roundtrip_mainnet() {
        let sut = AuthorizedDapps::sample_mainnet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
			[
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
									"account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
									"account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
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
										"00000000-0000-0000-0000-000000000001",
										"00000000-0000-0000-0000-000000000002"
									]
								},
								"phoneNumbers": {
									"request": {
										"quantifier": "atLeast",
										"quantity": 1
									},
									"ids": [
										"00000000-0000-0000-0000-000000000003",
										"00000000-0000-0000-0000-000000000004"
									]
								}
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
									"account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
								]
							},
							"sharedPersonaData": {
								"name": "00000000-0000-0000-0000-0000000000f0",
								"emailAddresses": {
									"request": {
										"quantifier": "exactly",
										"quantity": 2
									},
									"ids": [
										"00000000-0000-0000-0000-0000000000f1",
										"00000000-0000-0000-0000-0000000000f2"
									]
								},
								"phoneNumbers": {
									"request": {
										"quantifier": "atLeast",
										"quantity": 1
									},
									"ids": [
										"00000000-0000-0000-0000-0000000000f3",
										"00000000-0000-0000-0000-0000000000f4"
									]
								}
							}
						}
					]
				},
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
									"account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
								]
							},
							"sharedPersonaData": {
								"name": "00000000-0000-0000-0000-0000000000f0",
								"emailAddresses": {
									"request": {
										"quantifier": "exactly",
										"quantity": 2
									},
									"ids": [
										"00000000-0000-0000-0000-0000000000f1",
										"00000000-0000-0000-0000-0000000000f2"
									]
								},
								"phoneNumbers": {
									"request": {
										"quantifier": "atLeast",
										"quantity": 1
									},
									"ids": [
										"00000000-0000-0000-0000-0000000000f3",
										"00000000-0000-0000-0000-0000000000f4"
									]
								}
							}
						}
					]
				}
			]
            "#,
        );
    }

    #[test]
    fn json_roundtrip_stokenet() {
        let sut = AuthorizedDapps::sample_stokenet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
			[
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
							},
							"sharedPersonaData": {
								"name": "00000000-0000-0000-0000-000000000000",
								"emailAddresses": {
									"request": {
										"quantifier": "exactly",
										"quantity": 2
									},
									"ids": [
										"00000000-0000-0000-0000-000000000001",
										"00000000-0000-0000-0000-000000000002"
									]
								},
								"phoneNumbers": {
									"request": {
										"quantifier": "atLeast",
										"quantity": 1
									},
									"ids": [
										"00000000-0000-0000-0000-000000000003",
										"00000000-0000-0000-0000-000000000004"
									]
								}
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
							},
							"sharedPersonaData": {
								"name": "00000000-0000-0000-0000-0000000000f0",
								"emailAddresses": {
									"request": {
										"quantifier": "exactly",
										"quantity": 2
									},
									"ids": [
										"00000000-0000-0000-0000-0000000000f1",
										"00000000-0000-0000-0000-0000000000f2"
									]
								},
								"phoneNumbers": {
									"request": {
										"quantifier": "atLeast",
										"quantity": 1
									},
									"ids": [
										"00000000-0000-0000-0000-0000000000f3",
										"00000000-0000-0000-0000-0000000000f4"
									]
								}
							}
						}
					]
				},
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
							},
							"sharedPersonaData": {
								"name": "00000000-0000-0000-0000-0000000000f0",
								"emailAddresses": {
									"request": {
										"quantifier": "exactly",
										"quantity": 2
									},
									"ids": [
										"00000000-0000-0000-0000-0000000000f1",
										"00000000-0000-0000-0000-0000000000f2"
									]
								},
								"phoneNumbers": {
									"request": {
										"quantifier": "atLeast",
										"quantity": 1
									},
									"ids": [
										"00000000-0000-0000-0000-0000000000f3",
										"00000000-0000-0000-0000-0000000000f4"
									]
								}
							}
						}
					]
				}
			]
            "#,
        );
    }
}
