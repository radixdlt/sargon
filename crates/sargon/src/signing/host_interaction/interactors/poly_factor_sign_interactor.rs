use crate::prelude::*;

/// An interactor for a factor source kind which supports *Batch* usage of
/// multiple factor sources in poly.
///
/// Most FactorSourceKinds does in fact NOT support poly usage,
/// e.g. signing using multiple factors sources at once, but some do,
/// typically the DeviceFactorSource does, i.e. we can load multiple
/// mnemonics from secure storage in one go and sign with all of them
/// "in poly".
///
/// This is a bit of a misnomer, as we don't actually use them in poly,
/// but rather we iterate through all mnemonics and derive public keys/
/// or sign a payload with each of them in sequence
///
/// The user does not have the ability to SKIP a certain factor source,
/// instead either ALL factor sources are used to sign the transactions
/// or none.
///
/// Example of a PolyFactor Batch Signing Driver is that for DeviceFactorSource.
#[async_trait::async_trait]
pub trait PolyFactorSignInteractor<SP: SignablePayload> {
    async fn sign(
        &self,
        request: PolyFactorSignRequest<SP>,
    ) -> SignWithFactorsOutcome<SP::PayloadId>;
}
