use crate::prelude::*;

pub type SecurifiedAccounts = Entities<SecurifiedAccount>;
pub type SecurifiedPersonas = Entities<SecurifiedPersona>;

#[cfg(test)]
mod tests {
    // use super::*;
    // type Sut = SecurifiedAccounts;
    // type Item = SecurifiedAccount;
    // #[test]
    // fn empty_throws() {
    //     assert!(matches!(
    //         Sut::new(NetworkID::Mainnet, IndexSet::new()),
    //         Err(CommonError::EmptyCollection)
    //     ));
    // }
    // #[test]
    // fn wrong_network_single() {
    //     assert!(matches!(
    //         Sut::new(NetworkID::Stokenet, IndexSet::just(Item::sample())),
    //         Err(CommonError::WrongNetwork)
    //     ));
    // }
    // #[test]
    // fn wrong_network_two() {
    //     assert!(matches!(
    //         Sut::new(
    //             NetworkID::Stokenet,
    //             IndexSet::from_iter([Item::sample_other(), Item::sample(),])
    //         ),
    //         Err(CommonError::WrongNetwork)
    //     ));
    // }
    // #[test]
    // fn ok_new() {
    //     let network_id = NetworkID::Mainnet;
    //     let sut = Sut::new(network_id, IndexSet::just(Item::sample())).unwrap();
    //     assert!(!sut.is_empty());
    //     assert_eq!(sut.len(), 1);
    //     assert!(!sut.is_empty());
    //     assert_eq!(sut.network_id(), network_id);
    // }
}
