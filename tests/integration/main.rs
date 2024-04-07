mod network_antenna_reqwest;

#[cfg(test)]
mod integration_tests {

    use sargon::prelude::*;

    use crate::network_antenna_reqwest::new_gateway_client;

    #[actix_rt::test]
    async fn test_gateway() {
        let gateway = new_gateway_client(NetworkID::Mainnet);
        let xrd_balance = gateway
            .xrd_balance_of_account_or_zero(AccountAddress::sample_mainnet())
            .await
            .unwrap();
        assert!(xrd_balance >= Decimal192::one())
    }
}
