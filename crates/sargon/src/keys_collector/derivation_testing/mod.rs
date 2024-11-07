#[cfg(test)]
mod stateless_dummy_indices;
#[cfg(test)]
mod test_keys_collector;

#[cfg(test)]
pub(crate) use stateless_dummy_indices::*;
#[cfg(test)]
pub(crate) use test_keys_collector::*;

use crate::prelude::*;

#[cfg(test)]
pub fn do_derive_serially_looking_up_mnemonic_amongst_samples(
    request: MonoFactorKeyDerivationRequest,
    extra_mnemonics: IndexMap<FactorSourceIDFromHash, MnemonicWithPassphrase>,
) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
    __do_derive_serially_with_lookup_of_mnemonic(request, |f| {
        f.maybe_sample_associated_mnemonic()
            .or(extra_mnemonics.get(&f).cloned())
            .ok_or(CommonError::FactorSourceDiscrepancy)
    })
}

// FIXME put this behind a feature flag
pub fn __do_derive_serially_with_lookup_of_mnemonic(
    request: MonoFactorKeyDerivationRequest,
    lookup_mnemonic: impl Fn(
        FactorSourceIDFromHash,
    ) -> Result<MnemonicWithPassphrase>,
) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
    let factor_source_id = &request.factor_source_id;
    let instances = request
        .derivation_paths
        .into_iter()
        .map(|p| {
            let mnemonic = lookup_mnemonic(*factor_source_id)?;
            let seed = mnemonic.to_seed();
            let hd_private_key = seed.derive_private_key(&p);
            Ok(HierarchicalDeterministicFactorInstance::new(
                *factor_source_id,
                hd_private_key.public_key(),
            ))
        })
        .collect::<Result<IndexSet<_>>>()?;

    Ok(instances)
}
