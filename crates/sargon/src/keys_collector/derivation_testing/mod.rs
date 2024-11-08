#[cfg(test)]
mod stateless_dummy_indices;
#[cfg(test)]
mod test_keys_collector;

#[cfg(test)]
pub(crate) use stateless_dummy_indices::*;
#[cfg(test)]
pub(crate) use test_keys_collector::*;

use crate::prelude::*;
use std::future::ready;

#[cfg(test)]
pub async fn do_derive_serially_looking_up_mnemonic_amongst_samples<F>(
    request: MonoFactorKeyDerivationRequest,
    lookup_mnemonic: F,
) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>>
where
    F: async Fn(FactorSourceIDFromHash) -> Result<MnemonicWithPassphrase>,
{
    __do_derive_serially_with_lookup_of_mnemonic(
        request,
        async move |f: FactorSourceIDFromHash| {
            let res = lookup_mnemonic(f).await;
            return res.or(f
                .maybe_sample_associated_mnemonic()
                .ok_or(CommonError::FactorSourceDiscrepancy));
        },
    )
    .await
}

// FIXME put this behind a feature flag
pub async fn __do_derive_serially_with_lookup_of_mnemonic<F>(
    request: MonoFactorKeyDerivationRequest,
    lookup_mnemonic: F,
) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>>
where
    F: async Fn(FactorSourceIDFromHash) -> Result<MnemonicWithPassphrase>,
{
    let factor_source_id = request.factor_source_id;
    let mut out = IndexSet::<HierarchicalDeterministicFactorInstance>::new();

    for path in request.derivation_paths {
        let mnemonic = lookup_mnemonic(factor_source_id).await?;
        let seed = mnemonic.to_seed();
        let hd_private_key = seed.derive_private_key(&path);
        out.insert(HierarchicalDeterministicFactorInstance::new(
            factor_source_id,
            hd_private_key.public_key(),
        ));
    }
    Ok(out)
}
