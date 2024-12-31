use crate::prelude::*;

pub trait AuthorizedPersonaSimpleQueryState {
    fn accounts_for_display(
        &self,
        non_hidden_accounts: &Accounts,
    ) -> Result<Option<AccountsForDisplay>>;

    fn pick_persona_data_from_full(
        &self,
        full: &PersonaData,
    ) -> Result<PersonaData>;

    fn persona_from(&self, non_hidden_personas: &Personas) -> Result<Persona>;

    fn detailed(
        &self,
        non_hidden_personas: &Personas,
        non_hidden_accounts: &Accounts,
    ) -> Result<AuthorizedPersonaDetailed>;
}

impl AuthorizedPersonaSimpleQueryState for AuthorizedPersonaSimple {
    fn accounts_for_display(
        &self,
        non_hidden_accounts: &Accounts,
    ) -> Result<Option<AccountsForDisplay>> {
        let shared_accounts = self
            .shared_accounts
            .as_ref().map(|s| s.ids.clone())
            .unwrap_or_default()
            .iter()
            .map(|account_address| {
                let Some(account) = non_hidden_accounts
                    .iter().find(|x| x.address == account_address)
                else {
                        // This is a sign that Profile is in a bad state somehow...
                        warn!("Discrepancy! AuthorizedDapp references account which does not exist {}", account_address);
                        return Err(CommonError::DiscrepancyAuthorizedDappReferencedAccountWhichDoesNotExist {
                            address: account_address.to_owned().to_string()
                        })
                };
                Ok(AccountForDisplay::new(
                    account.address,
                    account.display_name,
                    account.appearance_id
                ))
            }).collect::<Result<AccountsForDisplay>>()?;

        if shared_accounts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(shared_accounts))
        }
    }

    fn pick_persona_data_from_full(
        &self,
        full: &PersonaData,
    ) -> Result<PersonaData> {
        let full_ids = &full.ids_of_entries();
        let shared = self.shared_persona_data.clone();
        let shared_ids = shared.ids_of_entries();

        if !full_ids.is_superset(&shared_ids) {
            error!("Profile discrepancy - most likely caused by incorrect implementation of DappInteractionFlow and updating of shared persona data. \n\nDetails [persona.personaData.ids] {:?} != {:?} [simple.sharedPersonaData]\n\npersona.personaData: {full}\n\nsimple.sharedPersonaData: {shared}", full_ids, shared_ids);
            return Err(
                CommonError::AuthorizedDappReferencesFieldIDThatDoesNotExist,
            );
        };

        let mut name: Option<PersonaDataIdentifiedName> = None;
        if let Some(saved_name) = &full.name {
            if let Some(shared) = shared.name {
                if shared.id() == saved_name.id {
                    name = Some(saved_name.clone());
                }
            };
        };

        let phone_numbers = full
            .phone_numbers
            .collection
            .clone()
            .into_iter()
            .filter(|x| {
                shared
                    .phone_numbers
                    .clone()
                    .map(|s| s.ids.clone())
                    .unwrap_or_default()
                    .into_iter()
                    .contains(&x.id)
            })
            .collect::<CollectionOfPhoneNumbers>();

        let email_addresses = full
            .email_addresses
            .collection
            .clone()
            .into_iter()
            .filter(|x| {
                shared
                    .email_addresses
                    .clone()
                    .map(|s| s.ids.clone())
                    .unwrap_or_default()
                    .into_iter()
                    .contains(&x.id)
            })
            .collect::<CollectionOfEmailAddresses>();

        Ok(PersonaData::new(name, phone_numbers, email_addresses))
    }

    fn persona_from(&self, non_hidden_personas: &Personas) -> Result<Persona> {
        let Some(persona) = non_hidden_personas
            .iter()
            .find(|x| x.address == self.identity_address)
        else {
            // This is a sign that Profile is in a bad state somehow...
            warn!("Discrepancy! AuthorizedDapp references persona which does not exist {}", self.identity_address);
            return Err(CommonError::DiscrepancyAuthorizedDappReferencedPersonaWhichDoesNotExist {
                address: self.identity_address.to_string()
            });
        };
        Ok(persona.clone())
    }

    fn detailed(
        &self,
        non_hidden_personas: &Personas,
        non_hidden_accounts: &Accounts,
    ) -> Result<AuthorizedPersonaDetailed> {
        let persona = self.persona_from(non_hidden_personas)?;
        let persona_data =
            self.pick_persona_data_from_full(&persona.persona_data)?;

        let shared_accounts = self.accounts_for_display(non_hidden_accounts)?;

        let has_auth_signing_key = persona.is_securified();
        Ok(AuthorizedPersonaDetailed::new(
            persona.address,
            persona.display_name,
            shared_accounts.clone(),
            persona_data,
            has_auth_signing_key,
        ))
    }
}

pub trait ProfileNetworkDetailsForAuthorizedDapp:
    ProfileNetworkEntitiesQuerying
{
    fn details_for_authorized_dapp(
        &self,
        dapp: &AuthorizedDapp,
    ) -> Result<AuthorizedDappDetailed>;
}

impl ProfileNetworkDetailsForAuthorizedDapp for ProfileNetwork {
    fn details_for_authorized_dapp(
        &self,
        dapp: &AuthorizedDapp,
    ) -> Result<AuthorizedDappDetailed> {
        self.is_on_same_network_as(dapp)?;

        let non_hidden_personas = &self.personas_non_hidden();
        let non_hidden_accounts = &self.accounts_non_hidden();

        let detailed_authorized_personas = dapp
            .references_to_authorized_personas
            .clone()
            .into_iter()
            .map(|simple| {
                simple.detailed(non_hidden_personas, non_hidden_accounts)
            })
            .collect::<Result<DetailedAuthorizedPersonas>>()?;

        Ok(AuthorizedDappDetailed::new(
            self.network_id(),
            dapp.dapp_definition_address,
            dapp.display_name
                .clone()
                .and_then(|x| DisplayName::new(x).ok()),
            detailed_authorized_personas,
            dapp.preferences.clone(),
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
                expected: sut.network_id().to_string(),
                actual: NetworkID::Stokenet.to_string()
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
			Err(CommonError::DiscrepancyAuthorizedDappReferencedPersonaWhichDoesNotExist { address: persona_simple.identity_address.to_string() })
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
			Err(CommonError::DiscrepancyAuthorizedDappReferencedAccountWhichDoesNotExist { address: address_of_non_existing_account.to_string() })
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
        let dapp = sut.authorized_dapps.first().unwrap();

        let details = sut.details_for_authorized_dapp(dapp).unwrap();
        assert_eq!(
            &details.dapp_definition_address,
            &dapp.dapp_definition_address
        );
        assert_eq!(&details.network_id, &dapp.network_id);

        assert_eq!(
            &details.display_name.map(|x| x.value()),
            &dapp.display_name
        );

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

        assert_eq!(&details.preferences, &dapp.preferences);
    }
}
