use crate::prelude::*;

impl Profile {
    pub fn sample_from<'a, 'p>(
        factor_sources: impl IntoIterator<Item = FactorSource>,
        accounts: impl IntoIterator<Item = &'a Account>,
        personas: impl IntoIterator<Item = &'p Persona>,
    ) -> Self {
        let mut networks = ProfileNetworks::new();

        accounts.into_iter().for_each(|a| {
            if networks.contains_id(a.network_id) {
                networks.update_with(a.network_id, |n| {
                    n.accounts.append(a.clone());
                });
            } else {
                let network = ProfileNetwork::new(
                    a.network_id,
                    Accounts::just(a.clone()),
                    Personas::new(),
                    AuthorizedDapps::default(),
                    ResourcePreferences::default(),
                    MFAFactorInstances::default(),
                );
                networks.append(network);
            }
        });

        personas.into_iter().for_each(|p| {
            if networks.contains_id(p.network_id) {
                networks.update_with(p.network_id, |n| {
                    n.personas.append(p.clone());
                });
            } else {
                let network = ProfileNetwork::new(
                    p.network_id,
                    Accounts::new(),
                    Personas::just(p.clone()),
                    AuthorizedDapps::default(),
                    ResourcePreferences::default(),
                    MFAFactorInstances::default(),
                );
                networks.append(network);
            }
        });

        Profile {
            header: Header::sample(),
            factor_sources: FactorSources::from_iter(factor_sources),
            app_preferences: Default::default(),
            networks,
        }
    }
}
