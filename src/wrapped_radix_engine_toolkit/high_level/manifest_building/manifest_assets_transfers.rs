use crate::prelude::*;

impl TransactionManifest {
    pub fn assets_transfers(
        transfers: AssetsTransfersPrototype,
        message: Message,
    ) -> Self {
        let mut builder = ScryptoManifestBuilder::new();


        for fungible in transfers.of_fungible_resources {
            // builder = builder.call_method(
            //     owner,
            //     ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
            //     (resource_address,),
            // )
            // let divisibility = resource.divisibility.map(UInt.init) ?? RETDecimal.maxDivisibility
            // try ManifestBuilder.withdrawAmount(
            //     accounts.fromAccount.address.intoEngine(),
            //     resource.address.intoEngine(),
            //     resource.totalTransferAmount.rounded(decimalPlaces: divisibility)
            // )
        }


        let scrypto_manifest = builder.build();

        TransactionManifest::from_scrypto(scrypto_manifest, owner.network_id())
    }
}
