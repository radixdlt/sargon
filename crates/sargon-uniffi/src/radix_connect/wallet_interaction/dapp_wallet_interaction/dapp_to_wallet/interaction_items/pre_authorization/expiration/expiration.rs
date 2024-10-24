use crate::prelude::*;
use sargon::DappToWalletInteractionSubintentExpiration as InternalDappToWalletInteractionSubintentExpiration;

/// An enum that represents the different ways a subintent can expire.
#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
pub enum DappToWalletInteractionSubintentExpiration {
    /// The subintent expires at a specific fixed timestamp.
    ///
    /// For example, a dApp sends a subintent for `User A` to approve sending 100 XRD before 5:00 PM,
    /// and a subintent for `User B` to approve sending 2 USDT with same expiration.
    ///
    /// If both users sign their subintents before 5:00 PM, the transaction to exchange
    /// 100 XRD over 2 USDT will succeed. Otherwise, it would fail.
    AtTime(DappToWalletInteractionSubintentExpireAtTime),

    /// The subintent expires X seconds after its signature.
    ///
    /// For example, a dApp sends a subintent for `User A` to approve sending 100 XRD with 1 hour expiration,
    /// and a subintent for `User B` to approve sending 2 USDT with same expiration.
    ///
    /// If both users sign their subintents within one hour from each other, the transaction to exchange
    /// 100 XRD over 2 USDT will succeed. Otherwise, it would fail.
    AfterDelay(DappToWalletInteractionSubintentExpireAfterDelay),
}
