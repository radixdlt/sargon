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

impl Into<Exactly32Bytes> for SignableManifestSummary {
    fn into(self) -> Exactly32Bytes {
        self.id.clone()
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

    fn sample_entities_requiring_auth<'a, 'p>(
        accounts_requiring_auth: impl IntoIterator<Item = &'a Account>,
        personas_requiring_auth: impl IntoIterator<Item = &'p Persona>,
    ) -> Self {
        Self::sample()
    }

    fn sample_entity_addresses_requiring_auth(
        account_addresses_requiring_auth: impl IntoIterator<Item = AccountAddress>,
        identity_addresses_requiring_auth: impl IntoIterator<Item = IdentityAddress>,
    ) -> Self {
        Self::sample()
    }

    fn sample_entity_addresses_with_pub_key_hashes_requiring_auth(
        account_addresses_requiring_auth: impl IntoIterator<
            Item = (AccountAddress, PublicKeyHash),
        >,
        identity_addresses_requiring_auth: impl IntoIterator<
            Item = (IdentityAddress, PublicKeyHash),
        >,
    ) -> Self {
        Self::sample()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signable_manifest_summary() {
        let summary = ManifestSummary::sample();
        let signable = SignableManifestSummary::new(summary.clone());

        assert_eq!(signable.summary, summary);
        assert_eq!(signable.entities_requiring_signing(&Profile::sample()), Ok(IndexSet::new()));
    }
}
