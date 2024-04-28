use crate::prelude::*;

trait Flatten<T> {
    fn flatten(self) -> Option<T>;
}

impl<T> Flatten<T> for Option<Option<T>> {
    fn flatten(self) -> Option<T> {
        match self {
            None => None,
            Some(v) => v,
        }
    }
}

impl PersonaData {
    pub fn ids_of_entries(&self) -> IndexSet<PersonaDataEntryID> {
        let mut full_ids = IndexSet::<PersonaDataEntryID>::new();
        if let Some(name) = &self.name {
            let _ = full_ids.insert(name.id);
        };
        full_ids.extend(self.email_addresses.ids());
        full_ids.extend(self.phone_numbers.ids());
        full_ids
    }
}

impl SharedPersonaData {
    pub fn ids_of_entries(&self) -> IndexSet<PersonaDataEntryID> {
        let mut full_ids = IndexSet::<PersonaDataEntryID>::new();
        if let Some(name) = &self.name {
            let _ = full_ids.insert(*name);
        };
        if let Some(email_addresses) = &self.email_addresses {
            full_ids.extend(email_addresses.clone().ids);
        }
        if let Some(phone_numbers) = &self.phone_numbers {
            full_ids.extend(phone_numbers.clone().ids);
        }
        full_ids
    }
}

impl ProfileNetwork {
    pub fn personas_non_hidden(&self) -> Personas {
        self.personas
            .clone()
            .into_iter()
            .filter(|p| !p.is_hidden())
            .collect()
    }

    pub fn accounts_non_hidden(&self) -> Accounts {
        self.accounts
            .clone()
            .into_iter()
            .filter(|p| !p.is_hidden())
            .collect()
    }

    pub fn details_for_authorized_dapp(
        &self,
        dapp: &AuthorizedDapp,
    ) -> Result<AuthorizedDappDetailed> {
        self.is_on_same_network_as(dapp)?;

        let detailed_authorized_personas = dapp
            .references_to_authorized_personas
			.clone()
            .into_iter()
            .map(|simple| {
                let Some(persona) = self
                    .personas_non_hidden()
                    .into_iter().find(|x| x.address == simple.identity_address)
                else {
					// This is a sign that Profile is in a bad state somehow...
					warn!("Discrepancy! AuthorizedDapp references persona which does not exist {}", simple.identity_address);
                    return Err(CommonError::DiscrepancyAuthorizedDappReferencedPersonaWhichDoesNotExist {
						address: simple.identity_address
					})
                };

				let display_name = persona.display_name.clone();
				let shared_accounts = simple
					.shared_accounts
					.map(|s| s.ids.clone())
					.unwrap_or_default()
					.into_iter()
					.map(|account_address| {
					    let Some(account) = self
							.accounts_non_hidden()
							.into_iter().find(|x| x.address == account_address)
						else {
								// This is a sign that Profile is in a bad state somehow...
								warn!("Discrepancy! AuthorizedDapp references account which does not exist {}", account_address);
								return Err(CommonError::DiscrepancyAuthorizedDappReferencedAccountWhichDoesNotExist {
									address: account_address
								})
						};
						Ok(AccountForDisplay::new(
							account.address,
							account.display_name,
							account.appearance_id
						))
					}).collect::<Result<AccountsForDisplay>>()?;
				let shared_accounts = if shared_accounts.is_empty() { None } else { Some(shared_accounts) };

				let full = persona.persona_data.clone();
				let full_ids = full.ids_of_entries();
				let shared = simple.shared_persona_data.clone();
				let shared_ids = shared.ids_of_entries();

				if !full_ids.is_superset(&shared_ids) {
					error!("Profile discrepancy - most likely caused by incorrect implementation of DappInteractionFlow and updating of shared persona data. \n\nDetails [persona.personaData.ids] {:?} != {:?} [simple.sharedPersonaData]\n\npersona.personaData: {full}\n\nsimple.sharedPersonaData: {shared}", full_ids, shared_ids);
					return Err(CommonError::AuthorizedDappReferencesFieldIDThatDoesNotExist)
				};

				let mut name: Option<PersonaDataIdentifiedName> = None;
				if let Some(saved_name) = full.name {
					if let Some(shared) = shared.name {
						if shared.id() == saved_name.id {
							name = Some(saved_name.clone());
						}
					};
				};

				let phone_numbers = full.phone_numbers.collection.clone().into_iter().filter(|x| {
					shared.phone_numbers.clone().map(|s| s.ids.clone()).unwrap_or_default().into_iter().contains(&x.id)
				}).collect::<CollectionOfPhoneNumbers>();

				let email_addresses = full.email_addresses.collection.clone().into_iter().filter(|x| {
					shared.email_addresses.clone().map(|s| s.ids.clone()).unwrap_or_default().into_iter().contains(&x.id)
				}).collect::<CollectionOfEmailAddresses>();

				let persona_data = PersonaData::new(name, phone_numbers, email_addresses);

				let has_auth_signing_key = match &persona.security_state {
					EntitySecurityState::Unsecured { value: uec } => uec.authentication_signing.is_some()
				};
				Ok(
					AuthorizedPersonaDetailed::new(
						persona.address,
						display_name,
						shared_accounts,
						persona_data,
						has_auth_signing_key
					)
				)
            })
            .collect::<Result<DetailedAuthorizedPersonas>>()?;

        Ok(AuthorizedDappDetailed::new(
            self.network_id(),
            dapp.dapp_definition_address,
            dapp.display_name
                .clone()
                .and_then(|x| DisplayName::new(x).ok()),
            detailed_authorized_personas,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProfileNetwork;

    #[test]
    fn network_discrepancy() {
        let sut = SUT::sample();
        assert_eq!(
            sut.details_for_authorized_dapp(&AuthorizedDapp::sample_stokenet()),
            Err(CommonError::NetworkDiscrepancy {
                expected: sut.network_id(),
                actual: NetworkID::Stokenet
            })
        )
    }

    #[test]
    fn details_persona_not_found() {
        let mut sut = SUT::sample();
        sut.personas = Personas::new();
        let mut dapp = AuthorizedDapp::sample();
        let persona_simple = AuthorizedPersonaSimple::sample();
        dapp.references_to_authorized_personas =
            ReferencesToAuthorizedPersonas::just(persona_simple.clone());

        assert_eq!(
			sut.details_for_authorized_dapp(&dapp),
			Err(CommonError::DiscrepancyAuthorizedDappReferencedPersonaWhichDoesNotExist { address: persona_simple.identity_address })
		);
    }

    #[test]
    fn details_account_not_found() {
        let mut sut = SUT::sample();
        let persona = Persona::sample();
        sut.personas = Personas::just(persona.clone());
        sut.accounts = Accounts::new();
        let mut dapp = AuthorizedDapp::sample();
        let address_of_non_existing_account = AccountAddress::sample();

        let persona_simple = AuthorizedPersonaSimple::new(
            persona.address,
            now(),
            Some(SharedToDappWithPersonaAccountAddresses::just(
                address_of_non_existing_account,
            )),
            SharedPersonaData::default(),
        );
        dapp.references_to_authorized_personas =
            ReferencesToAuthorizedPersonas::just(persona_simple.clone());

        assert_eq!(
			sut.details_for_authorized_dapp(&dapp),
			Err(CommonError::DiscrepancyAuthorizedDappReferencedAccountWhichDoesNotExist { address: address_of_non_existing_account })
		);
    }

    #[test]
    fn details_shared_field_does_not_exist() {
        let mut sut = SUT::sample();
        let mut persona = Persona::sample();
        persona.persona_data = PersonaData::default(); // empty
        let account = Account::sample();
        sut.personas = Personas::just(persona.clone());
        sut.accounts = Accounts::just(account.clone());
        let mut dapp = AuthorizedDapp::sample();
        let id_of_non_existing_persona_data_field =
            PersonaDataEntryID::sample();

        let persona_simple = AuthorizedPersonaSimple::new(
            persona.address,
            now(),
            Some(SharedToDappWithPersonaAccountAddresses::just(
                account.address,
            )),
            SharedPersonaData::new(
                id_of_non_existing_persona_data_field,
                None,
                None,
            ),
        );

        dapp.references_to_authorized_personas =
            ReferencesToAuthorizedPersonas::just(persona_simple.clone());

        assert_eq!(
            sut.details_for_authorized_dapp(&dapp),
            Err(CommonError::AuthorizedDappReferencesFieldIDThatDoesNotExist)
        );
    }

    #[test]
    fn details_ok() {
        let sut = SUT::sample();
        let dapp = sut.authorized_dapps.first().clone().unwrap();

        let details = sut.details_for_authorized_dapp(&dapp).unwrap();
        assert_eq!(
            &details.dapp_definition_address,
            &dapp.dapp_definition_address
        );
        assert_eq!(&details.network_id, &dapp.network_id);

        assert_eq!(&details.display_name.map(|x| x.value), &dapp.display_name);

        assert_eq!(
            &details
                .detailed_authorized_personas
                .clone()
                .into_iter()
                .find(|d| d.identity_address
                    == Persona::sample_mainnet_satoshi().address)
                .unwrap()
                .shared_persona_data,
            &Persona::sample_mainnet_satoshi().persona_data
        );

        assert_eq!(
            &details
                .detailed_authorized_personas
                .clone()
                .into_iter()
                .find(|d| d.identity_address
                    == Persona::sample_mainnet_batman().address)
                .unwrap()
                .shared_persona_data,
            &Persona::sample_mainnet_batman().persona_data
        );
    }
}
