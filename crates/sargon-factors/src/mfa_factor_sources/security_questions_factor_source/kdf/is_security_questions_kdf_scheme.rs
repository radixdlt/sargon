use crate::prelude::*;

pub trait IsSecurityQuestionsKDFScheme {
    fn derive_encryption_keys_from_questions_and_answers(
        &self,
        questions_and_answers: Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
    ) -> Result<Vec<EncryptionKey>>;
}
