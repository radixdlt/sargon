use crate::prelude::*;

pub struct TestSpotCheckInteractor {
    user: SpotCheckUser,
}

#[async_trait::async_trait]
impl SpotCheckInteractor for TestSpotCheckInteractor {
    async fn spot_check(
        &self,
        _factor_source_id: FactorSourceIDFromHash,
    ) -> Result<SpotCheckResponse> {
        match self.user.clone() {
            SpotCheckUser::Failure(common_error) => Err(common_error),
            SpotCheckUser::Ledger(id) => Ok(SpotCheckResponse::Ledger { id }),
            SpotCheckUser::ArculusCard(id) => {
                Ok(SpotCheckResponse::ArculusCard { id })
            }
            SpotCheckUser::MnemonicWithPassphrase(value) => {
                Ok(SpotCheckResponse::MnemonicWithPassphrase { value })
            }
        }
    }
}

#[derive(Clone)]
pub enum SpotCheckUser {
    Failure(CommonError),
    Ledger(Exactly32Bytes),
    ArculusCard(FactorSourceIDFromHash),
    MnemonicWithPassphrase(MnemonicWithPassphrase),
}

impl TestSpotCheckInteractor {
    pub fn new(user: SpotCheckUser) -> Self {
        Self { user }
    }
    pub fn new_failing() -> Self {
        Self::new(SpotCheckUser::Failure(CommonError::Unknown))
    }

    pub fn new_failing_error(common_error: CommonError) -> Self {
        Self::new(SpotCheckUser::Failure(common_error))
    }

    pub fn new_ledger(id: Exactly32Bytes) -> Self {
        Self::new(SpotCheckUser::Ledger(id))
    }

    pub fn new_arculus_card(id: FactorSourceIDFromHash) -> Self {
        Self::new(SpotCheckUser::ArculusCard(id))
    }

    pub fn new_mnemonic_with_passphrase(
        mnemonic_with_passphrase: MnemonicWithPassphrase,
    ) -> Self {
        Self::new(SpotCheckUser::MnemonicWithPassphrase(
            mnemonic_with_passphrase,
        ))
    }
}
