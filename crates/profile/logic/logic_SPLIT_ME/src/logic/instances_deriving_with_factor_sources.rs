// use crate::prelude::*;

// trait InstancesDerivingWithFactorSources {
//     fn derive_instances_for_factor_sources(
//         network_id: NetworkID,
//         quantity_per_factor: usize,
//         derivation_presets: impl IntoIterator<Item = DerivationPreset>,
//         sources: impl IntoIterator<Item = FactorSource>,
//     ) -> IndexMap<FactorSourceIDFromHash, FactorInstances>;
// }

// impl InstancesDerivingWithFactorSources for MnemonicWithPassphrase {
//     fn derive_instances_for_factor_sources(
//         network_id: NetworkID,
//         quantity_per_factor: usize,
//         derivation_presets: impl IntoIterator<Item = DerivationPreset>,
//         sources: impl IntoIterator<Item = FactorSource>,
//     ) -> IndexMap<FactorSourceIDFromHash, FactorInstances> {
//         let next_index_assigner = NextDerivationEntityIndexAssigner::new(
//             network_id,
//             None,
//             FactorInstancesCache::default(),
//         );

//         let derivation_presets =
//             derivation_presets.into_iter().collect::<Vec<_>>();

//         sources
//             .into_iter()
//             .map(|fs| {
//                 let fsid = fs.id_from_hash();
//                 let mwp = fsid.sample_associated_mnemonic();

//                 let paths = derivation_presets
//                     .clone()
//                     .into_iter()
//                     .map(|dp| (dp, quantity_per_factor))
//                     .collect::<IndexMap<DerivationPreset, usize>>();

//                 let paths = paths
//                     .into_iter()
//                     .flat_map(|(derivation_preset, qty)| {
//                         // `qty` many paths
//                         (0..qty)
//                             .map(|_| {
//                                 let index_agnostic_path = derivation_preset
//                                     .index_agnostic_path_on_network(network_id);

//                                 next_index_assigner
//                                     .next(fsid, index_agnostic_path)
//                                     .map(|index| {
//                                         DerivationPath::from_index_agnostic_path_and_component(
//                                             index_agnostic_path,
//                                             index,
//                                         )
//                                     })
//                                     .unwrap()
//                             })
//                             .collect::<IndexSet<DerivationPath>>()
//                     })
//                     .collect::<IndexSet<DerivationPath>>();

//                 let instances = mwp
//                     .derive_public_keys(paths)
//                     .into_iter()
//                     .map(|public_key| {
//                         HierarchicalDeterministicFactorInstance::new(
//                             fsid, public_key,
//                         )
//                     })
//                     .collect::<FactorInstances>();

//                 (fsid, instances)
//             })
//             .collect::<IndexMap<FactorSourceIDFromHash, FactorInstances>>()
//     }
// }
