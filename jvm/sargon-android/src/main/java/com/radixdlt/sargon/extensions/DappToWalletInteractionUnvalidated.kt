package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DappToWalletInteractionUnvalidated
import com.radixdlt.sargon.dappToWalletInteractionUnvalidatedToJsonBytes
import com.radixdlt.sargon.newDappToWalletInteractionUnvalidatedFromJsonBytes

fun DappToWalletInteractionUnvalidated.Companion.fromJson(json: String) = runCatching {
    newDappToWalletInteractionUnvalidatedFromJsonBytes(json.toByteArray().toBagOfBytes())
}
fun DappToWalletInteractionUnvalidated.toJson() = runCatching {
    dappToWalletInteractionUnvalidatedToJsonBytes(this).string
}

