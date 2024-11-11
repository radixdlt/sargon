package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DappToWalletInteractionSubintentExpiration
import com.radixdlt.sargon.DappToWalletInteractionSubintentExpirationStatus
import com.radixdlt.sargon.getSubintentExpirationStatus

val DappToWalletInteractionSubintentExpiration.status: DappToWalletInteractionSubintentExpirationStatus
    get() = getSubintentExpirationStatus(expiration = this)

