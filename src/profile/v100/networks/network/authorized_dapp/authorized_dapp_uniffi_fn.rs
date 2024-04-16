use crate::prelude::*;

#[uniffi::export]
pub fn new_authorized_dapp_sample_mainnet_dashboard() -> AuthorizedDapp {
    AuthorizedDapp::sample_mainnet_dashboard()
}

#[uniffi::export]
pub fn new_authorized_dapp_sample_mainnet_gumballclub() -> AuthorizedDapp {
    AuthorizedDapp::sample_mainnet_gumballclub()
}

#[uniffi::export]
pub fn new_authorized_dapp_sample_stokenet_devconsole() -> AuthorizedDapp {
    AuthorizedDapp::sample_stokenet_devconsole()
}

#[uniffi::export]
pub fn new_authorized_dapp_sample_stokenet_sandbox() -> AuthorizedDapp {
    AuthorizedDapp::sample_stokenet_sandbox()
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthorizedDapp;

    #[test]
    fn samples() {
        assert_eq!(
            new_authorized_dapp_sample_mainnet_dashboard(),
            SUT::sample_mainnet_dashboard()
        );

        assert_eq!(
            new_authorized_dapp_sample_mainnet_gumballclub(),
            SUT::sample_mainnet_gumballclub()
        );

        assert_eq!(
            new_authorized_dapp_sample_stokenet_devconsole(),
            SUT::sample_stokenet_devconsole()
        );

        assert_eq!(
            new_authorized_dapp_sample_stokenet_sandbox(),
            SUT::sample_stokenet_sandbox()
        );
    }
}
