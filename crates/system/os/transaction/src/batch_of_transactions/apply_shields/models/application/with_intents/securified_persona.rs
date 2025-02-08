use crate::{
    create_application_for_securified_entity_with_intents, prelude::*,
};

create_application_for_securified_entity_with_intents! {
    SecurityShieldApplicationTransactionIntentsForSecurifiedPersona,
    /// The persona we are applyying the shield for
    /// and the account that will pay the topping up of the AccessControllers XRD vault.
    persona_with_paying_account: SecurityShieldApplicationForSecurifiedPersonaWithPayingAccount
}
