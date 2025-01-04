use crate::prelude::*;

pub trait ReferencedAccountRemoving {
    fn remove_referenced_account(&mut self, account_address: &AccountAddress);
}

impl ReferencedAccountRemoving for AuthorizedDapps {
    /// Remove referenced account from all the dApps
    fn remove_referenced_account(&mut self, account_address: &AccountAddress) {
        self.update_all_with(|dapp| {
            dapp.remove_referenced_account(account_address);
        })
    }
}

pub trait AuthorizedDappUpdating {
    /// Removes the referenced account for this dApp
    fn remove_referenced_account(&mut self, account_address: &AccountAddress);
}

impl AuthorizedDappUpdating for AuthorizedDapp {
    /// Removes the referenced account for this dApp
    fn remove_referenced_account(&mut self, account_address: &AccountAddress) {
        self.references_to_authorized_personas
            .update_all_with(|persona| {
                persona.remove_shared_account(account_address);
            });
    }
}
