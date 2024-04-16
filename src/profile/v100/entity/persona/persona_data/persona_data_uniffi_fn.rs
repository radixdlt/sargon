use crate::prelude::*;

#[uniffi::export]
pub fn new_persona_data_sample() -> PersonaData {
    PersonaData::sample()
}

#[uniffi::export]
pub fn new_persona_data_sample_other() -> PersonaData {
    PersonaData::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        assert_eq!(new_persona_data_sample(), PersonaData::sample());

        assert_eq!(
            new_persona_data_sample_other(),
            PersonaData::sample_other()
        );
    }
}
