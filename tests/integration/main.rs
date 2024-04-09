mod network_antenna_reqwest;

#[cfg(test)]
mod integration_tests {

    use std::time::Duration;

    use actix_rt::time::timeout;
    use sargon::prelude::*;
    use std::collections::HashMap;

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
    async fn test_xrd_balance_of_account_or_zero_is_zero_for_unknown_mainnet() {
        let network_id = NetworkID::Mainnet;
        let gateway_client = new_gateway_client(network_id);
        let sut = gateway_client
            .xrd_balance_of_account_or_zero(AccountAddress::random(network_id));

        let xrd_balance = timeout(MAX, sut).await.unwrap().unwrap();
        assert_eq!(xrd_balance, Decimal192::zero());
    }

    #[actix_rt::test]
    async fn test_xrd_balance_of_account_or_zero_is_zero_for_unknown_stokenet()
    {
        let network_id = NetworkID::Stokenet;
        let gateway_client = new_gateway_client(network_id);
        let sut = gateway_client
            .xrd_balance_of_account_or_zero(AccountAddress::random(network_id));

        let xrd_balance = timeout(MAX, sut).await.unwrap().unwrap();
        assert_eq!(xrd_balance, Decimal192::zero());
    }

    #[actix_rt::test]
    async fn test_epoch() {
        let gateway_client = new_gateway_client(NetworkID::Mainnet);
        let sut = gateway_client.current_epoch();
        let epoch = timeout(MAX, sut).await.unwrap().unwrap();
        assert!(epoch > Epoch::from(0));
    }

    #[actix_rt::test]
    async fn transaction_dry_run() {
        // ARRANGE
        let network_id = NetworkID::Mainnet;
        let gateway_client = new_gateway_client(network_id);
        let start_epoch_inclusive =
            timeout(MAX, gateway_client.current_epoch())
                .await
                .unwrap()
                .unwrap();

        let from = AccountAddress::sample_mainnet();
        let to = AccountAddress::sample_mainnet_other();
        let resource = ResourceAddress::sample();
        let amount = Decimal192::one();
        let transfers = PerRecipientAssetTransfers::new(
            from,
            [PerRecipientAssetTransfer::new(
                to,
                [PerRecipientFungibleTransfer::new(
                    resource, amount, true, None,
                )],
                [],
            )],
        );

        let manifest = TransactionManifest::per_recipient_transfers(transfers);

        let end_epoch_exclusive = Epoch::from(start_epoch_inclusive.0 + 10u64);
        let notary_public_key = Ed25519PublicKey::sample();
        let header = TransactionHeader::new(
            network_id,
            start_epoch_inclusive,
            end_epoch_exclusive,
            Nonce::random(),
            notary_public_key,
            true,
            0,
        );

        let intent =
            TransactionIntent::new(header, manifest.clone(), Message::None)
                .unwrap();

        let sut = gateway_client.transaction_dry_run(
            intent, vec![
                    Ed25519PublicKey::from_hex(
                        "48d24f09b43d50f3acd58cf8509a57c8f306d94b945bd9b7e6ebcf6691eed3b6".to_owned()
                    ).unwrap().into()
                ]
            );

        // ACT
        let encoded_receipt = timeout(MAX, sut).await.unwrap().unwrap();
        let execution_summary =
            manifest.execution_summary(encoded_receipt).unwrap();

        // ASSERT
        assert_eq!(
            execution_summary.addresses_of_accounts_requiring_auth,
            vec![from]
        );
        assert_eq!(
            execution_summary.deposits,
            HashMap::<_, _>::from_iter([(
                to,
                vec![ResourceIndicator::fungible(
                    resource,
                    FungibleResourceIndicator::guaranteed(amount)
                )]
            )])
        );
    }
}
