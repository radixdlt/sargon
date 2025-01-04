use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedAuthIntent {
    pub intent: AuthIntent,
    pub intent_signatures_per_owner:
        IndexMap<AddressOfAccountOrPersona, IntentSignature>,
}

impl SignedAuthIntent {
    pub fn new(
        intent: AuthIntent,
        intent_signatures_per_owner: IndexMap<
            AddressOfAccountOrPersona,
            IntentSignature,
        >,
    ) -> Result<Self> {
        let all_intent_signatures = IntentSignatures::new(
            intent_signatures_per_owner.values().cloned().collect_vec(),
        );
        if !all_intent_signatures.validate(intent.auth_intent_hash()) {
            return Err(CommonError::InvalidSignaturesForIntentSomeDidNotValidateIntentHash);
        }

        Ok(Self {
            intent,
            intent_signatures_per_owner,
        })
    }

    pub fn intent(&self) -> &AuthIntent {
        &self.intent
    }
}

impl HasSampleValues for SignedAuthIntent {
    fn sample() -> Self {
        let intent = AuthIntent::sample();
        let mnemonic_with_passphrase = MnemonicWithPassphrase::sample();

        let signature = mnemonic_with_passphrase
            .sign(&intent.auth_intent_hash().hash(), &DerivationPath::sample());
        let intent_signatures = IndexMap::kv(
            AddressOfAccountOrPersona::sample(),
            IntentSignature(signature),
        );

        SignedAuthIntent::new(intent, intent_signatures).unwrap()
    }

    fn sample_other() -> Self {
        let intent = AuthIntent::sample_other();
        let mnemonic_with_passphrase = MnemonicWithPassphrase::sample();

        let signature = mnemonic_with_passphrase
            .sign(&intent.auth_intent_hash().hash(), &DerivationPath::sample());
        let intent_signatures = IndexMap::kv(
            AddressOfAccountOrPersona::sample(),
            IntentSignature(signature),
        );

        SignedAuthIntent::new(intent, intent_signatures).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignedAuthIntent;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn test_valid_signatures() {
        let intent = AuthIntent::sample();
        let mnemonic_with_passphrase = MnemonicWithPassphrase::sample();

        let signature = mnemonic_with_passphrase
            .sign(&intent.auth_intent_hash().hash(), &DerivationPath::sample());
        let intent_signatures = IndexMap::kv(
            AddressOfAccountOrPersona::sample(),
            IntentSignature(signature),
        );
        assert!(SignedAuthIntent::new(intent, intent_signatures).is_ok())
    }

    #[test]
    fn test_invalid_signatures() {
        let intent_signatures = IndexMap::kv(
            AddressOfAccountOrPersona::sample(),
            IntentSignature::sample(),
        );
        assert_eq!(
            SUT::new(AuthIntent::sample(), intent_signatures),
            Err(CommonError::InvalidSignaturesForIntentSomeDidNotValidateIntentHash)
        )
    }

    #[test]
    fn test_get_intent() {
        let intent = AuthIntent::sample();
        let mnemonic_with_passphrase = MnemonicWithPassphrase::sample();

        let signature = mnemonic_with_passphrase
            .sign(&intent.auth_intent_hash().hash(), &DerivationPath::sample());
        let intent_signatures = IndexMap::kv(
            AddressOfAccountOrPersona::sample(),
            IntentSignature(signature),
        );

        assert_eq!(
            SUT::new(intent.clone(), intent_signatures)
                .unwrap()
                .intent()
                .clone(),
            intent
        )
    }
}
