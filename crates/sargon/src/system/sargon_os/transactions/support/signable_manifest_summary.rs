use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignableManifestSummary {
    pub id: Exactly32Bytes,
    pub summary: ManifestSummary,
}

impl SignableManifestSummary {
    pub fn new(summary: ManifestSummary) -> Self {
        Self {
            id: Exactly32Bytes::generate(),
            summary: summary,
        }
    }
}

impl HasSampleValues for SignableManifestSummary {
    fn sample() -> Self {
        Self {
            id: Exactly32Bytes::sample(),
            summary: ManifestSummary::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            id: Exactly32Bytes::sample_other(),
            summary: ManifestSummary::sample_other(),
        }
    }
}

impl SignableID for Exactly32Bytes {}

impl From<SignableManifestSummary> for Exactly32Bytes {
    fn from(val: SignableManifestSummary) -> Exactly32Bytes {
        val.id.clone()
    }
}

impl std::hash::Hash for SignableManifestSummary {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Signable for SignableManifestSummary {
    type ID = Exactly32Bytes;
    type Payload = Self;

    fn entities_requiring_signing(
        &self,
        profile: &Profile,
    ) -> Result<IndexSet<AccountOrPersona>> {
        ExtractorOfEntitiesRequiringAuth::extract(profile, self.summary.clone())
    }

    fn get_id(&self) -> Self::ID {
        self.id.clone()
    }

    fn get_payload(&self) -> Self::Payload {
        panic!("Manifest summary cannot be actually signed")
    }

    fn sample_entity_addresses_with_pub_key_hashes(
        all_addresses_with_hashes: Vec<(
            AddressOfAccountOrPersona,
            PublicKeyHash,
        )>,
        network_id: Option<NetworkID>,
    ) -> Self {
        let mut builder = ScryptoTransactionManifestBuilder::new();
        let network_id = network_id.unwrap_or_default();

        for (address, hash) in all_addresses_with_hashes {
            builder = builder.set_metadata(
                address.scrypto(),
                MetadataKey::OwnerKeys,
                ScryptoMetadataValue::PublicKeyHashArray(vec![hash.into()]),
            );
        }

        let manifest = TransactionManifest::sargon_built(builder, network_id);

        Self::new(manifest.summary().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type SUT = SignableManifestSummary;

    #[test]
    fn test_eq() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn test_api() {
        let profile = Profile::sample();
        let account = Account::sample_mainnet();
        let persona = Persona::sample_mainnet();

        let manifest = TransactionManifest::set_owner_keys_hashes(
            &persona.address.into(),
            vec![PublicKeyHash::sample()],
        )
        .modify_add_lock_fee(&account.address, Some(Decimal192::one()));
        let summary = manifest.summary().unwrap();
        let signable = SUT::new(summary.clone());

        assert_eq!(signable.summary, summary);
        assert_eq!(
            signable.entities_requiring_signing(&profile),
            Ok(IndexSet::from_iter(vec![
                AccountOrPersona::from(account),
                AccountOrPersona::from(persona),
            ]))
        );
    }
}
