use crate::prelude::*;

pub trait HostInteractor:
    SignInteractor<TransactionIntent> +
    SignInteractor<Subintent> +
    KeysDerivationInteractors {}