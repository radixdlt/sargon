use crate::prelude::*;
use transaction::model::{
    NotarizedTransactionV1 as ScryptoNotarizedTransaction,
    SignedIntentV1 as ScryptoSignedIntent,
};

#[derive(Debug, Clone, Eq, PartialEq, uniffi::Record)]
pub struct NotarizedTransaction {
    pub signed_intent: SignedIntent,
    pub notary_signature: NotarySignature,
}

impl NotarizedTransaction {
    pub fn new(
        signed_intent: SignedIntent,
        notary_signature: NotarySignature,
    ) -> Self {
        Self {
            signed_intent,
            notary_signature,
        }
    }

    pub fn compile(&self) -> Result<BagOfBytes> {
        todo!()
    }
}

impl From<NotarizedTransaction> for ScryptoNotarizedTransaction {
    fn from(value: NotarizedTransaction) -> Self {
        ScryptoNotarizedTransaction {
            signed_intent: value.signed_intent.into(),
            notary_signature: value.notary_signature.into(),
        }
    }
}

impl TryFrom<ScryptoNotarizedTransaction> for NotarizedTransaction {
    type Error = crate::CommonError;

    fn try_from(
        value: ScryptoNotarizedTransaction,
    ) -> Result<Self, Self::Error> {
        let signed_intent: SignedIntent = value.signed_intent.try_into()?;
        Ok(Self {
            signed_intent,
            notary_signature: value.notary_signature.into(),
        })
    }
}

impl HasSampleValues for NotarizedTransaction {
    fn sample() -> Self {
        todo!()
    }

    // Identical to: https://github.com/radixdlt/radixdlt-scrypto/blob/ff21f24952318387803ae720105eec079afe33f3/transaction/src/model/hash/encoder.rs#L115
    // intent hash: `"60e5617d670e6c8a42ba5f3749f4ff1079f66221f282554ecdda9ad385ecb195"`
    // bech32 encoded   (mainnet): `"txid_rdx1vrjkzlt8pekg5s46tum5na8lzpulvc3p72p92nkdm2dd8p0vkx2syss63y"`
    // bech32 encoded (simulator): `"txid_sim1vrjkzlt8pekg5s46tum5na8lzpulvc3p72p92nkdm2dd8p0vkx2svr7ejr"`
    fn sample_other() -> Self {
        let secret_key: Secp256k1PrivateKey =
            radix_engine::types::Secp256k1PrivateKey::from_u64(1)
                .unwrap()
                .into();
        // let network_id = NetworkID::Simulator;
        // let header = TransactionHeader {
        //     network_id,
        //     start_epoch_inclusive: 0.into(),
        //     end_epoch_exclusive: 10.into(),
        //     nonce: 10.into(),
        //     notary_is_signatory: true,
        //     notary_public_key: pk.public_key().into(),
        //     tip_percentage: 0,
        // };
        // let intent = TransactionIntent::new(
        //     header,
        //     TransactionManifest::empty(network_id),
        //     Message::None,
        // );
        let intent = TransactionIntent::sample_other();
        assert_eq!(intent.intent_hash().unwrap().to_string(), "txid_sim1vrjkzlt8pekg5s46tum5na8lzpulvc3p72p92nkdm2dd8p0vkx2svr7ejr");
        let signed_intent =
            SignedIntent::new(intent, IntentSignatures::new(Vec::new()));

        let signed_intent_hash = signed_intent.hash().unwrap();
        let notary_signature_secp = secret_key.sign(&signed_intent_hash.hash);
        let notary_signature: NotarySignature = notary_signature_secp.into();

        NotarizedTransaction::new(signed_intent, notary_signature)
    }
}
