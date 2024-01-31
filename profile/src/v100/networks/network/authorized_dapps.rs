use crate::prelude::*;

/// An ordered set of Authorized Dapps on a specific network.
pub type AuthorizedDapps = IdentifiedVecVia<AuthorizedDapp>;

impl AuthorizedDapps {
    /// Instantiates a new collection of authorized_dapps from
    /// and iterator of authorized_dapps.
    pub fn with_authorized_dapps<I>(authorized_dapps: I) -> Self
    where
        I: IntoIterator<Item = AuthorizedDapp>,
    {
        Self::from_iter(authorized_dapps)
    }

    /// Instantiates a new collection of authorized_dapps from a
    /// single authorized_dapp.
    pub fn with_authorized_dapp(authorized_dapp: AuthorizedDapp) -> Self {
        Self::with_authorized_dapps([authorized_dapp])
    }
}

// Trait: Default
impl Default for AuthorizedDapps {
    /// Instantiates a new empty networks collection.
    fn default() -> Self {
        Self::new()
    }
}

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

impl HasPlaceholder for AuthorizedDapps {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::placeholder_mainnet()
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::placeholder_stokenet()
    }
}

impl AuthorizedDapps {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_mainnet() -> Self {
        Self::with_authorized_dapps([
            AuthorizedDapp::placeholder_mainnet_dashboard(),
            AuthorizedDapp::placeholder_mainnet_gumballclub(),
        ])
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_stokenet() -> Self {
        Self::with_authorized_dapps([
            AuthorizedDapp::placeholder_stokenet_devconsole(),
            AuthorizedDapp::placeholder_stokenet_sandbox(),
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
        assert_ne!(
            AuthorizedDapps::placeholder(),
            AuthorizedDapps::placeholder_other()
        );
    }

    #[test]
    fn equality() {
        assert_eq!(
            AuthorizedDapps::placeholder(),
            AuthorizedDapps::placeholder()
        );
        assert_eq!(
            AuthorizedDapps::placeholder_other(),
            AuthorizedDapps::placeholder_other()
        );
    }

    #[test]
    fn duplicates_are_prevented() {
        assert_eq!(
            AuthorizedDapps::with_authorized_dapps(
                [AuthorizedDapp::placeholder(), AuthorizedDapp::placeholder()]
                    .into_iter()
            )
            .len(),
            1
        )
    }

    #[test]
    fn with_one() {
        assert_eq!(AuthorizedDapps::with_authorized_dapp(AuthorizedDapp::placeholder()).len(), 1)
    }

    #[test]
    fn get_all() {
        assert_eq!(AuthorizedDapps::placeholder().get_all().len(), 2);
    }

    #[test]
    fn get_by_address() {
        let authorized_dapp = AuthorizedDapp::placeholder();
        let address = authorized_dapp.dapp_definition_address.clone();
        let authorized_dapps =
            AuthorizedDapps::with_authorized_dapp(authorized_dapp.clone());
        assert_eq!(
            authorized_dapps.get_authorized_dapp_by_address(&address),
            Some(&authorized_dapp)
        );
    }

    #[test]
    fn json_roundtrip_mainnet() {
        let sut = AuthorizedDapps::placeholder_mainnet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
                
            ]
            "#,
        );
    }

    #[test]
    fn json_roundtrip_stokenet() {
        let sut = AuthorizedDapps::placeholder_stokenet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
			[
				
			]
            "#,
        );
    }
}
