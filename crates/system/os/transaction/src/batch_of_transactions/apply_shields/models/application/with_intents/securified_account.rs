use crate::{
    create_application_for_securified_entity_with_intents, prelude::*,
};

create_application_for_securified_entity_with_intents! {
    SecurityShieldApplicationTransactionIntentsForSecurifiedAccount,
    /// The account we are applying the shield for and an optional other payer
    account_with_optional_paying_account: SecurityShieldApplicationForSecurifiedAccountWithOptionalPayingAccount
}
