#[derive(
    Debug, Clone, Copy, PartialEq, Eq, InternalConversion, uniffi::enum,
)]
pub enum OwnedOrThirdPartyAccountAddress {
    OwnedAccount { value: AccountAddress },
    ThirdPartyAccount { value: AccountAddress },
}
