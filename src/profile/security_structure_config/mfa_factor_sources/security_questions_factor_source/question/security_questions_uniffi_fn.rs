use crate::prelude::*;

#[uniffi::export]
pub fn security_questions_all() -> Security_NOT_PRODUCTION_READY_Questions {
    Security_NOT_PRODUCTION_READY_Questions::from_iter(
        Security_NOT_PRODUCTION_READY_Question::all().into_iter(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        assert_eq!(security_questions_all().len(), 17);
    }
}
