package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DappToWalletInteractionUnvalidated
import com.radixdlt.sargon.newDappToWalletInteractionUnvalidatedFromJsonBytes

fun DappToWalletInteractionUnvalidated.Companion.init(json: String) = runCatching {
    newDappToWalletInteractionUnvalidatedFromJsonBytes(json.toByteArray().toBagOfBytes())
}

