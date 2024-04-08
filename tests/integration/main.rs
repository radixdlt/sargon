mod network_antenna_reqwest;

#[cfg(test)]
mod integration_tests {

    use std::time::Duration;

    use actix_rt::time::timeout;
    use sargon::prelude::*;

    use crate::network_antenna_reqwest::new_gateway_client;

    const MAX: Duration = Duration::from_secs(2);

    #[actix_rt::test]
    async fn test_xrd_balance_of_account_or_zero() {
        let gateway_client = new_gateway_client(NetworkID::Mainnet);
        let sut = gateway_client
            .xrd_balance_of_account_or_zero(AccountAddress::sample_mainnet());

        let xrd_balance = timeout(MAX, sut).await.unwrap().unwrap();
        assert!(xrd_balance >= Decimal192::one())
    }

    #[actix_rt::test]
    async fn test_epoch() {
        let gateway_client = new_gateway_client(NetworkID::Mainnet);
        let sut = gateway_client.current_epoch();
        let epoch = timeout(MAX, sut).await.unwrap().unwrap();
        assert!(epoch > Epoch::from(0));
    }
}
