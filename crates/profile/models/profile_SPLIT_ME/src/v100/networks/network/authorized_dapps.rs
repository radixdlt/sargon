use crate::prelude::*;

decl_identified_vec_of!(
    /// An ordered set of ['AuthorizedDapp`]s on a specific network.
    AuthorizedDapp
);

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

impl HasSampleValuesOnNetworks for AuthorizedDapps {
    /// A sample used to facilitate unit tests.
    fn sample_mainnet() -> Self {
        Self::from_iter([
            AuthorizedDapp::sample_mainnet_dashboard(),
            AuthorizedDapp::sample_mainnet_gumballclub(),
        ])
    }

    /// A sample used to facilitate unit tests.
    fn sample_stokenet() -> Self {
        Self::from_iter([
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
            AuthorizedDapps::from_iter(
                [AuthorizedDapp::sample(), AuthorizedDapp::sample()]
                    .into_iter()
            )
            .len(),
            1
        )
    }

    #[test]
    fn with_one() {
        assert_eq!(AuthorizedDapps::just(AuthorizedDapp::sample()).len(), 1)
    }

    #[test]
    fn get_all() {
        assert_eq!(AuthorizedDapps::sample().get_all().len(), 2);
    }

    #[test]
    fn get_by_address() {
        let authorized_dapp = AuthorizedDapp::sample();
        let address = authorized_dapp.dapp_definition_address;
        let authorized_dapps = AuthorizedDapps::just(authorized_dapp.clone());
        assert_eq!(authorized_dapps.get_id(address), Some(&authorized_dapp));
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
				},
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
				},
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
			]
            "#,
        );
    }
}
